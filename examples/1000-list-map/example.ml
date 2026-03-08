(* OCaml List mapping examples: idiomatic and recursive implementations *)

(* === IDIOMATIC OCAML === *)
(* This is the standard OCaml approach using List.map *)
let map_iter_demo () =
  let numbers = [1; 2; 3; 4; 5] in
  let doubled = List.map (fun x -> x * 2) numbers in
  List.iter (fun x -> Printf.printf "%d " x) doubled;
  Printf.printf "\n"

(* === RECURSIVE OCAML === *)
(* This demonstrates a functional recursive approach *)
let rec map_recursive f = function
  | [] -> []
  | x :: xs -> f x :: map_recursive f xs

let map_recursive_demo () =
  let numbers = [1; 2; 3; 4; 5] in
  let doubled = map_recursive (fun x -> x * 2) numbers in
  List.iter (fun x -> Printf.printf "%d " x) doubled;
  Printf.printf "\n"

(* === TAIL-RECURSIVE OCAML === *)
(* This is more efficient due to tail-call optimization *)
let map_tail_recursive f xs =
  let rec go acc = function
    | [] -> List.rev acc
    | x :: xs -> go (f x :: acc) xs
  in
  go [] xs

let map_tail_recursive_demo () =
  let numbers = [1; 2; 3; 4; 5] in
  let doubled = map_tail_recursive (fun x -> x * 2) numbers in
  List.iter (fun x -> Printf.printf "%d " x) doubled;
  Printf.printf "\n"

(* === TESTS === *)
let test_idiomatic_empty () =
  let result = List.map (fun x -> x * 2) [] in
  assert (result = [])

let test_idiomatic_single () =
  let result = List.map (fun x -> x * 2) [5] in
  assert (result = [10])

let test_idiomatic_multiple () =
  let result = List.map (fun x -> x * 2) [1; 2; 3; 4; 5] in
  assert (result = [2; 4; 6; 8; 10])

let test_idiomatic_negative () =
  let result = List.map (fun x -> abs x) [-1; -2; -3] in
  assert (result = [1; 2; 3])

let test_idiomatic_type_conversion () =
  let result = List.map (fun x -> string_of_int x) [1; 2; 3] in
  assert (result = ["1"; "2"; "3"])

let test_recursive_empty () =
  let result = map_recursive (fun x -> x * 2) [] in
  assert (result = [])

let test_recursive_single () =
  let result = map_recursive (fun x -> x * 2) [5] in
  assert (result = [10])

let test_recursive_multiple () =
  let result = map_recursive (fun x -> x * 2) [1; 2; 3; 4; 5] in
  assert (result = [2; 4; 6; 8; 10])

let test_recursive_squares () =
  let result = map_recursive (fun x -> x * x) [1; 2; 3; 4; 5] in
  assert (result = [1; 4; 9; 16; 25])

let test_tail_recursive_multiple () =
  let result = map_tail_recursive (fun x -> x * 2) [1; 2; 3; 4; 5] in
  assert (result = [2; 4; 6; 8; 10])

let test_tail_recursive_squares () =
  let result = map_tail_recursive (fun x -> x * x) [1; 2; 3; 4; 5] in
  assert (result = [1; 4; 9; 16; 25])

(* Run all tests *)
let () =
  Printf.printf "Running OCaml tests...\n";
  test_idiomatic_empty ();
  test_idiomatic_single ();
  test_idiomatic_multiple ();
  test_idiomatic_negative ();
  test_idiomatic_type_conversion ();
  test_recursive_empty ();
  test_recursive_single ();
  test_recursive_multiple ();
  test_recursive_squares ();
  test_tail_recursive_multiple ();
  test_tail_recursive_squares ();
  Printf.printf "All tests passed!\n\n";

  Printf.printf "=== Idiomatic OCaml (List.map) ===\n";
  map_iter_demo ();

  Printf.printf "\n=== Recursive OCaml ===\n";
  map_recursive_demo ();

  Printf.printf "\n=== Tail-Recursive OCaml ===\n";
  map_tail_recursive_demo ()
