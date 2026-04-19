(* List.filter — select elements of a list that satisfy a predicate. *)

let numbers = [1; 2; 3; 4; 5; 6; 7; 8]

(* Idiomatic OCaml — standard library [List.filter]. *)
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds  = List.filter (fun x -> x mod 2 <> 0) numbers

(* Partial application: [pos] is a function [int list -> int list]. *)
let pos = List.filter (fun n -> n > 0)

(* Recursive implementation — shows the pattern matching explicitly. *)
let rec filter_rec p = function
  | [] -> []
  | x :: xs ->
    if p x then x :: filter_rec p xs
    else filter_rec p xs

let () =
  assert (evens = [2; 4; 6; 8]);
  assert (odds  = [1; 3; 5; 7]);
  assert (pos numbers = [1; 2; 3; 4; 5; 6; 7; 8]);
  assert (filter_rec (fun n -> n > 5) numbers = [6; 7; 8]);
  assert (List.filter (fun _ -> false) numbers = []);
  assert (List.filter (fun _ -> true)  numbers = numbers);
  print_endline "ok"
