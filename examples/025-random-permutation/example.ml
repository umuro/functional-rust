(* Random Permutation *)
(* OCaml 99 Problems #25 *)

(* Fisher-Yates shuffle: uniform random permutation, O(n) *)
let permutation lst =
  let arr = Array.of_list lst in
  let n = Array.length arr in
  for i = n - 1 downto 1 do
    let j = Random.int (i + 1) in
    let tmp = arr.(i) in
    arr.(i) <- arr.(j);
    arr.(j) <- tmp
  done;
  Array.to_list arr

(* Tests *)
let () =
  Random.init 1;
  assert (List.length (permutation [1; 2; 3; 4; 5]) = 5);

  Random.init 2;
  let source = [1; 2; 3; 4; 5] in
  let shuffled = permutation source in
  assert (List.sort compare shuffled = List.sort compare source);

  Random.init 42;
  let a = permutation [1; 2; 3; 4; 5] in
  Random.init 42;
  let b = permutation [1; 2; 3; 4; 5] in
  assert (a = b);

  assert (permutation [] = []);
  assert (permutation [42] = [42]);

  print_endline "✓ OCaml tests passed"
