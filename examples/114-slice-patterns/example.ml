(* Example 114: Slice Patterns — OCaml List Pattern Matching in Rust *)

(* Approach 1: Head/tail pattern matching *)
let rec sum = function
  | [] -> 0
  | x :: rest -> x + sum rest

let rec take n = function
  | _ when n <= 0 -> []
  | [] -> []
  | x :: rest -> x :: take (n - 1) rest

let approach1 () =
  assert (sum [1; 2; 3; 4; 5] = 15);
  assert (take 3 [1; 2; 3; 4; 5] = [1; 2; 3]);
  Printf.printf "Sum: %d, Take 3: %s\n" (sum [1;2;3;4;5])
    (String.concat "," (List.map string_of_int (take 3 [1;2;3;4;5])))

(* Approach 2: Matching specific patterns *)
let describe = function
  | [] -> "empty"
  | [_] -> "singleton"
  | [_; _] -> "pair"
  | _ :: _ :: _ -> "many"

let approach2 () =
  assert (describe [] = "empty");
  assert (describe [1] = "singleton");
  assert (describe [1; 2] = "pair");
  assert (describe [1; 2; 3] = "many");
  Printf.printf "Patterns work!\n"

(* Approach 3: First, last, and middle *)
let first_and_last lst =
  match lst with
  | [] -> None
  | [x] -> Some (x, x)
  | x :: rest ->
    let last = List.nth rest (List.length rest - 1) in
    Some (x, last)

let approach3 () =
  assert (first_and_last [] = None);
  assert (first_and_last [1] = Some (1, 1));
  assert (first_and_last [1; 2; 3] = Some (1, 3));
  Printf.printf "First and last work!\n"

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
