(* 978: Count-Min Sketch *)
(* Frequency estimation with O(1) update/query, O(d*w) space *)
(* Uses d hash functions × w counters; estimates freq with min over d rows *)

(* Simple hash functions using different seeds *)
let make_hash seed =
  fun s ->
    String.fold_left (fun h c ->
      h * seed lxor Char.code c
    ) seed s

(* Create d different hash functions *)
let make_hashes d =
  let seeds = [| 31; 37; 41; 43; 47; 53; 59; 61 |] in
  Array.init d (fun i -> make_hash seeds.(i mod Array.length seeds))

type sketch = {
  table: int array array;  (* d rows × w columns *)
  hashes: (string -> int) array;
  width: int;
  depth: int;
}

let create ~width ~depth =
  { table = Array.make_matrix depth width 0;
    hashes = make_hashes depth;
    width;
    depth }

let update sk key delta =
  for i = 0 to sk.depth - 1 do
    let col = abs (sk.hashes.(i) key) mod sk.width in
    sk.table.(i).(col) <- sk.table.(i).(col) + delta
  done

let query sk key =
  let min_count = ref max_int in
  for i = 0 to sk.depth - 1 do
    let col = abs (sk.hashes.(i) key) mod sk.width in
    let v = sk.table.(i).(col) in
    if v < !min_count then min_count := v
  done;
  !min_count

let () =
  let sk = create ~width:100 ~depth:5 in

  (* Update with counts *)
  for _ = 1 to 100 do update sk "apple" 1 done;
  for _ = 1 to 50  do update sk "banana" 1 done;
  for _ = 1 to 25  do update sk "cherry" 1 done;

  (* Estimates are >= actual (may overestimate due to hash collisions) *)
  let apple_est = query sk "apple" in
  let banana_est = query sk "banana" in
  let cherry_est = query sk "cherry" in

  assert (apple_est >= 100);
  assert (banana_est >= 50);
  assert (cherry_est >= 25);

  (* Never underestimate (count-min property) *)
  assert (apple_est >= 100);

  (* Unseen item estimate should be close to 0 *)
  let unseen = query sk "dragon" in
  assert (unseen < 10);  (* should be very low for 100 wide, 5 deep *)

  Printf.printf "apple:  %d (actual 100)\n" apple_est;
  Printf.printf "banana: %d (actual 50)\n" banana_est;
  Printf.printf "cherry: %d (actual 25)\n" cherry_est;
  Printf.printf "dragon: %d (actual 0)\n" unseen;
  Printf.printf "✓ All tests passed\n"
