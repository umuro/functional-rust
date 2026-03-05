(* Visitor Pattern via Fold — Expression Evaluator *)

type expr =
  | Lit of float
  | Add of expr * expr
  | Mul of expr * expr
  | Neg of expr

let rec fold ~lit ~add ~mul ~neg = function
  | Lit x -> lit x
  | Add (a, b) -> add (fold ~lit ~add ~mul ~neg a) (fold ~lit ~add ~mul ~neg b)
  | Mul (a, b) -> mul (fold ~lit ~add ~mul ~neg a) (fold ~lit ~add ~mul ~neg b)
  | Neg a -> neg (fold ~lit ~add ~mul ~neg a)

let eval = fold ~lit:Fun.id ~add:(+.) ~mul:( *.) ~neg:(fun x -> -.x)

let to_string = fold
  ~lit:string_of_float
  ~add:(fun a b -> "(" ^ a ^ " + " ^ b ^ ")")
  ~mul:(fun a b -> "(" ^ a ^ " * " ^ b ^ ")")
  ~neg:(fun a -> "(-" ^ a ^ ")")

let () =
  let e = Add (Mul (Lit 2.0, Lit 3.0), Neg (Lit 1.0)) in
  assert (eval e = 5.0);
  assert (to_string e = "((2. * 3.) + (-1.))");
  Printf.printf "%s = %g\n" (to_string e) (eval e);
  print_endline "ok"
