(* 355: Linked List
   OCaml's native list is an immutable singly-linked list — the most
   natural data structure in the language. All operations are O(n)
   except prepend (O(1)). Rust's LinkedList is rarely used; same advice
   applies in OCaml: prefer arrays or sequences for most use cases. *)

(* Build a list from items — trivial in OCaml *)
let build_list items = items  (* OCaml lists ARE linked lists *)

(* Concatenate two lists — O(n) where n = length of first list *)
let concat a b = a @ b

(* Split list at position [at] *)
let split_at lst at =
  let rec go front rest n =
    match rest, n with
    | _, 0 -> (List.rev front, rest)
    | [], _ -> (List.rev front, [])
    | x :: xs, _ -> go (x::front) xs (n-1)
  in
  go [] lst at

let () =
  (* Build and iterate *)
  let lst = build_list [1;2;3] in
  assert (lst = [1;2;3]);
  Printf.printf "list: [%s]\n%!"
    (lst |> List.map string_of_int |> String.concat "; ");

  (* Concat *)
  let a = build_list [1;2] in
  let b = build_list [3;4] in
  let c = concat a b in
  assert (c = [1;2;3;4]);
  Printf.printf "concat: [%s]\n%!"
    (c |> List.map string_of_int |> String.concat "; ");

  (* Split *)
  let lst2 = build_list [1;2;3;4;5] in
  let (p, q) = split_at lst2 2 in
  assert (p = [1;2]);
  assert (q = [3;4;5]);
  Printf.printf "split at 2: [%s] | [%s]\n%!"
    (p |> List.map string_of_int |> String.concat "; ")
    (q |> List.map string_of_int |> String.concat "; ");

  (* Pattern matching — idiomatic OCaml list usage *)
  let rec sum = function
    | []      -> 0
    | x :: xs -> x + sum xs
  in
  assert (sum [1;2;3;4;5] = 15);
  Printf.printf "sum via pattern match: %d\n%!" (sum [1;2;3;4;5])
