let rec is_even = function
  | 0 -> true
  | n -> is_odd (n - 1)
and     is_odd = function
  | 0 -> false
  | n -> is_even (n - 1)

type expr =
  | Lit of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval_expr = function
  | Lit n      -> n
  | Add (l, r) -> eval_expr l + eval_expr r
  | Mul (l, r) -> eval_mul l r
and eval_mul l r =
  eval_expr l * eval_expr r

let () =
  assert (is_even 4 = true);
  assert (is_odd 7 = true);
  assert (is_even 0 = true);
  let e = Mul (Add (Lit 2, Lit 3), Lit 4) in
  assert (eval_expr e = 20);
  print_endline "All assertions passed."
