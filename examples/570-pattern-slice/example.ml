(* List patterns in OCaml *)
let rec sum = function
  | []      -> 0
  | x :: xs -> x + sum xs

let rec product = function
  | []      -> 1
  | x :: xs -> x * product xs

let first_two = function
  | a :: b :: _ -> Some (a, b)
  | _           -> None

let describe = function
  | []          -> "empty"
  | [x]         -> Printf.sprintf "one: %d" x
  | [a;b]       -> Printf.sprintf "pair: (%d,%d)" a b
  | h :: _      -> Printf.sprintf "many, first: %d" h

let () =
  let xs = [1;2;3;4;5] in
  Printf.printf "sum=%d product=%d\n" (sum xs) (product xs);
  (match first_two xs with Some(a,b)->Printf.printf "(%d,%d)\n" a b | None->());
  Printf.printf "%s\n" (describe xs)
