(* Lotto Draw *)
(* OCaml 99 Problems #24 *)

let range a b =
  let rec aux acc n = if n < a then acc else aux (n :: acc) (n - 1) in
  aux [] b

let random_select lst n =
  let arr = Array.of_list lst in
  let len = Array.length arr in
  let n = min n len in
  for i = 0 to n - 1 do
    let j = i + Random.int (len - i) in
    let tmp = arr.(i) in
    arr.(i) <- arr.(j);
    arr.(j) <- tmp
  done;
  Array.to_list (Array.sub arr 0 n)

let lotto_select n m = List.sort compare (random_select (range 1 m) n)

(* Tests *)
let () =
  Random.init 42;
  let draw = lotto_select 6 49 in
  assert (List.length draw = 6);
  assert (draw = List.sort compare draw);
  assert (List.for_all (fun x -> x >= 1 && x <= 49) draw);
  assert (List.length (List.sort_uniq compare draw) = 6);

  let draw2 = lotto_select 10 5 in
  assert (draw2 = [1; 2; 3; 4; 5]);

  Random.init 123;
  let a = lotto_select 6 49 in
  Random.init 123;
  let b = lotto_select 6 49 in
  assert (a = b);

  print_endline "✓ OCaml tests passed"
