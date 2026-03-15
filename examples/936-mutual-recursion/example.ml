(* 936: Mutual Recursion with `and`

   OCaml uses `let rec ... and ...` for mutually recursive functions.
   This is explicit — the compiler needs to know the full group up front.
   Rust allows any function to call any other in scope without special syntax. *)

(* ── Mutually recursive even/odd check ───────────────────────────────────── *)

(* In OCaml, `and` binds the two definitions into one mutual recursion group *)
let rec is_even n =
  match n with
  | 0 -> true
  | n -> is_odd (n - 1)

and is_odd n =
  match n with
  | 0 -> false
  | n -> is_even (n - 1)

(* ── Expression tree with mutual recursion over variants ─────────────────── *)

type expr =
  | Lit of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval_expr = function
  | Lit n      -> n
  | Add (l, r) -> eval_expr l + eval_expr r
  | Mul (l, r) -> eval_mul l r

and eval_mul l r = eval_expr l * eval_expr r

(* ── Mutually recursive data: rose tree / forest ─────────────────────────── *)

type 'a rose_tree = Tree  of 'a * 'a forest
and  'a forest    = 'a rose_tree list

let rec tree_size (Tree (_, children)) = 1 + forest_size children
and     forest_size ts = List.fold_left (fun acc t -> acc + tree_size t) 0 ts

(* Sum all values in a rose tree *)
let rec tree_sum (Tree (v, children)) = v + forest_sum children
and     forest_sum ts = List.fold_left (fun acc t -> acc + tree_sum t) 0 ts

(* ── Iterative fallback for large n ──────────────────────────────────────── *)

let is_even_fast n = n mod 2 = 0

let () =
  assert (is_even 4);
  assert (not (is_even 7));
  assert (is_even 0);
  assert (is_odd 7);
  assert (not (is_odd 4));
  assert (not (is_odd 0));

  (* (2 + 3) * 4 = 20 *)
  let e = Mul (Add (Lit 2, Lit 3), Lit 4) in
  assert (eval_expr e = 20);

  assert (eval_expr (Lit 42) = 42);
  assert (eval_expr (Add (Lit 1, Lit 2)) = 3);

  (* (1 + 2) * (3 + 4) = 21 *)
  let e2 = Mul (Add (Lit 1, Lit 2), Add (Lit 3, Lit 4)) in
  assert (eval_expr e2 = 21);

  (* Rose tree mutual recursion *)
  let t =
    Tree (1, [
      Tree (2, [Tree (4, []); Tree (5, [])]);
      Tree (3, [Tree (6, [])])
    ])
  in
  assert (tree_size t = 6);
  assert (tree_sum t = 21);

  (* fast even matches slow *)
  assert (is_even_fast 100);
  assert (not (is_even_fast 101));

  print_endline "936-mutual-recursion: all tests passed"
