(* Example 177: GADT Typed Expression Evaluator *)
(* A fully typed expression language where only well-typed expressions compile *)

(* Approach 1: Complete typed expression evaluator *)
type _ expr =
  | Lit    : int -> int expr
  | BLit   : bool -> bool expr
  | Add    : int expr * int expr -> int expr
  | Mul    : int expr * int expr -> int expr
  | Eq     : int expr * int expr -> bool expr
  | And    : bool expr * bool expr -> bool expr
  | Not    : bool expr -> bool expr
  | If     : bool expr * 'a expr * 'a expr -> 'a expr
  | Pair   : 'a expr * 'b expr -> ('a * 'b) expr
  | Fst    : ('a * 'b) expr -> 'a expr
  | Snd    : ('a * 'b) expr -> 'b expr

let rec eval : type a. a expr -> a = function
  | Lit n -> n
  | BLit b -> b
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b
  | Eq (a, b) -> eval a = eval b
  | And (a, b) -> eval a && eval b
  | Not a -> not (eval a)
  | If (c, t, f) -> if eval c then eval t else eval f
  | Pair (a, b) -> (eval a, eval b)
  | Fst p -> fst (eval p)
  | Snd p -> snd (eval p)

(* Approach 2: Pretty printer that preserves type info *)
let rec to_string : type a. a expr -> string = function
  | Lit n -> string_of_int n
  | BLit b -> string_of_bool b
  | Add (a, b) -> "(" ^ to_string a ^ " + " ^ to_string b ^ ")"
  | Mul (a, b) -> "(" ^ to_string a ^ " * " ^ to_string b ^ ")"
  | Eq (a, b) -> "(" ^ to_string a ^ " = " ^ to_string b ^ ")"
  | And (a, b) -> "(" ^ to_string a ^ " && " ^ to_string b ^ ")"
  | Not a -> "not(" ^ to_string a ^ ")"
  | If (c, t, f) -> "if " ^ to_string c ^ " then " ^ to_string t ^ " else " ^ to_string f
  | Pair (a, b) -> "(" ^ to_string a ^ ", " ^ to_string b ^ ")"
  | Fst p -> "fst(" ^ to_string p ^ ")"
  | Snd p -> "snd(" ^ to_string p ^ ")"

(* Approach 3: Constant folding optimizer *)
let rec optimize : type a. a expr -> a expr = function
  | Add (Lit 0, b) -> optimize b
  | Add (a, Lit 0) -> optimize a
  | Mul (Lit 0, _) -> Lit 0
  | Mul (_, Lit 0) -> Lit 0
  | Mul (Lit 1, b) -> optimize b
  | Mul (a, Lit 1) -> optimize a
  | Add (Lit a, Lit b) -> Lit (a + b)
  | Mul (Lit a, Lit b) -> Lit (a * b)
  | And (BLit true, b) -> optimize b
  | And (_, BLit false) -> BLit false
  | Not (BLit b) -> BLit (not b)
  | If (BLit true, t, _) -> optimize t
  | If (BLit false, _, f) -> optimize f
  | e -> e

let () =
  (* Test evaluation *)
  assert (eval (Lit 42) = 42);
  assert (eval (Add (Lit 1, Lit 2)) = 3);
  assert (eval (Mul (Lit 3, Lit 4)) = 12);
  assert (eval (Eq (Lit 1, Lit 1)) = true);
  assert (eval (Eq (Lit 1, Lit 2)) = false);
  assert (eval (And (BLit true, BLit true)) = true);
  assert (eval (Not (BLit true)) = false);
  assert (eval (If (BLit true, Lit 10, Lit 20)) = 10);
  assert (eval (Pair (Lit 1, BLit true)) = (1, true));
  assert (eval (Fst (Pair (Lit 1, BLit true))) = 1);
  assert (eval (Snd (Pair (Lit 1, BLit true))) = true);

  (* Test pretty printing *)
  assert (to_string (Add (Lit 1, Lit 2)) = "(1 + 2)");

  (* Test optimizer *)
  assert (eval (optimize (Add (Lit 0, Lit 5))) = 5);
  assert (eval (optimize (Mul (Lit 0, Lit 999))) = 0);
  assert (eval (optimize (If (BLit true, Lit 1, Lit 2))) = 1);

  print_endline "✓ All tests passed"
