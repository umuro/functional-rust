(* 966: Fenwick Tree (Binary Indexed Tree)
   O(log n) prefix-sum queries and O(log n) point updates.
   More cache-friendly and simpler to implement than a segment tree
   when only prefix sums are needed.
   Key trick: index i is responsible for the range ending at i whose
   length is the lowest set bit of i. *)

(* 1-indexed internally; external interface is 0-indexed *)
type fenwick = { tree : int array; n : int }

let create n =
  assert (n > 0);
  { tree = Array.make (n + 1) 0; n }

let create_of_array arr =
  let n = Array.length arr in
  let ft = create n in
  (* O(n) build using the "add each element" approach *)
  Array.iteri (fun i v ->
    let i = i + 1 in   (* 1-indexed *)
    let j = ref i in
    while !j <= n do
      ft.tree.(!j) <- ft.tree.(!j) + v;
      j := !j + (!j land (- !j))  (* add lowest set bit *)
    done
  ) arr;
  ft

(* Add delta to position i (0-indexed) *)
let update ft i delta =
  assert (i >= 0 && i < ft.n);
  let j = ref (i + 1) in
  while !j <= ft.n do
    ft.tree.(!j) <- ft.tree.(!j) + delta;
    j := !j + (!j land (- !j))
  done

(* Prefix sum [0, i] inclusive (0-indexed) *)
let prefix_sum ft i =
  assert (i >= 0 && i < ft.n);
  let sum = ref 0 in
  let j = ref (i + 1) in
  while !j > 0 do
    sum := !sum + ft.tree.(!j);
    j := !j - (!j land (- !j))   (* remove lowest set bit *)
  done;
  !sum

(* Range sum [l, r] inclusive (0-indexed) *)
let range_sum ft l r =
  assert (l <= r);
  if l = 0 then prefix_sum ft r
  else prefix_sum ft r - prefix_sum ft (l - 1)

(* Point query at index i (current value) *)
let point_query ft i = range_sum ft i i

(* Find the smallest index with prefix_sum >= target (binary lifting) *)
let lower_bound ft target =
  let pos = ref 0 and sum = ref 0 in
  let bit = ref (1 lsl (int_of_float (log (float_of_int ft.n) /. log 2.0))) in
  while !bit > 0 do
    let next = !pos + !bit in
    if next <= ft.n && !sum + ft.tree.(next) < target then begin
      pos := next;
      sum := !sum + ft.tree.(next)
    end;
    bit := !bit asr 1
  done;
  !pos   (* 0-indexed result *)

let () =
  let arr = [|2; 1; 4; 3; 5; 8; 6; 7|] in
  let ft = create_of_array arr in

  Printf.printf "Prefix sums:\n";
  Printf.printf "  [0,0] = %d\n" (prefix_sum ft 0);   (* 2 *)
  Printf.printf "  [0,3] = %d\n" (prefix_sum ft 3);   (* 10 *)
  Printf.printf "  [0,7] = %d\n" (prefix_sum ft 7);   (* 36 *)

  Printf.printf "Range sums:\n";
  Printf.printf "  [2,5] = %d\n" (range_sum ft 2 5);  (* 20 *)
  Printf.printf "  [1,3] = %d\n" (range_sum ft 1 3);  (* 8 *)

  update ft 3 10;  (* arr[3] was 3, now 13 *)
  Printf.printf "\nAfter update idx=3 by +10:\n";
  Printf.printf "  prefix_sum[0,3] = %d\n" (prefix_sum ft 3);  (* 20 *)
  Printf.printf "  range_sum[2,5]  = %d\n" (range_sum ft 2 5); (* 30 *)

  Printf.printf "\nPoint queries:\n";
  Array.iteri (fun i _ ->
    Printf.printf "  arr[%d] = %d\n" i (point_query ft i)
  ) arr
