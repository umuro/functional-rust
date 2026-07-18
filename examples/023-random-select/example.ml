(* Random Select *)
(* OCaml 99 Problems #23 *)

(* Fisher-Yates partial shuffle: swap the i-th element with a random
   element in [i, len), building the selected prefix. O(k). *)
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

(* Tests *)
let () =
  Random.init 42;
  let selected = random_select [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] 4 in
  assert (List.length selected = 4);

  let source = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in
  let selected2 = random_select source 5 in
  let unique = List.sort_uniq compare selected2 in
  assert (List.length unique = 5);
  assert (List.for_all (fun x -> List.mem x source) selected2);

  assert (random_select [1; 2; 3] 0 = []);

  let selected3 = random_select [1; 2; 3] 10 in
  assert (List.length selected3 = 3);
  assert (List.length (List.sort_uniq compare selected3) = 3);

  Random.init 99;
  let a = random_select [1; 2; 3; 4; 5] 3 in
  Random.init 99;
  let b = random_select [1; 2; 3; 4; 5] 3 in
  assert (a = b);

  print_endline "✓ OCaml tests passed"
