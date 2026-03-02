(* Palindrome Check *)
(* OCaml 99 Problems #6 *)

(* Solution 1: Using List.rev *)
let is_palindrome lst =
  lst = List.rev lst

(* Solution 2: Manual comparison *)
let is_palindrome_manual lst =
  let rec compare l1 l2 =
    match l1, l2 with
    | [], [] -> true
    | _, [] | [], _ -> false
    | h1 :: t1, h2 :: t2 -> h1 = h2 && compare t1 t2
  in
  compare lst (List.rev lst)

(* Tests *)
let () =
  assert (is_palindrome [1; 2; 3; 2; 1] = true);
  assert (is_palindrome [1; 2; 3; 4] = false);
  assert (is_palindrome [] = true);
  assert (is_palindrome [1] = true);
  print_endline "✓ OCaml tests passed"
