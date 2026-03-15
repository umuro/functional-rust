(* Example 224: Mutumorphism — Genuinely Mutual Recursion *)

(* mutu : ('f ('a,'b) -> 'a) -> ('f ('a,'b) -> 'b) -> fix -> ('a, 'b)
   Two folds that depend on EACH OTHER. Unlike zygo where the helper is independent. *)

type 'a nat_f = ZeroF | SuccF of 'a

let map_nat f = function ZeroF -> ZeroF | SuccF a -> SuccF (f a)

type fix_nat = FixN of fix_nat nat_f

(* Approach 1: isEven / isOdd — classic mutual recursion *)
(* isEven 0 = true,  isEven (n+1) = isOdd n
   isOdd 0  = false, isOdd  (n+1) = isEven n *)

let rec mutu alg_a alg_b (FixN f) =
  let paired = map_nat (mutu alg_a alg_b) f in
  (alg_a (map_nat fst paired), alg_b (map_nat snd paired))
(* Hmm, this maps twice. Better: *)

let rec mutu alg_a alg_b (FixN f) =
  let paired = map_nat (fun child -> mutu alg_a alg_b child) f in
  (alg_a paired, alg_b paired)
(* But paired has type (a*b) nat_f, and alg_a needs it too *)

(* Correct implementation *)
let rec mutu (alg_a : ('a * 'b) nat_f -> 'a) (alg_b : ('a * 'b) nat_f -> 'b) (FixN f : fix_nat) : ('a * 'b) =
  let paired = map_nat (mutu alg_a alg_b) f in
  (alg_a paired, alg_b paired)

let is_even_alg : (bool * bool) nat_f -> bool = function
  | ZeroF -> true
  | SuccF (_even, odd) -> odd  (* isEven(n+1) = isOdd(n) *)

let is_odd_alg : (bool * bool) nat_f -> bool = function
  | ZeroF -> false
  | SuccF (even, _odd) -> even  (* isOdd(n+1) = isEven(n) *)

let zero = FixN ZeroF
let succ n = FixN (SuccF n)
let rec nat n = if n <= 0 then zero else succ (nat (n - 1))

let is_even n = fst (mutu is_even_alg is_odd_alg (nat n))
let is_odd n = snd (mutu is_even_alg is_odd_alg (nat n))

(* Approach 2: Collatz conjecture — count steps AND track max *)
(* But with natural numbers: parity-check + step-count *)

(* Approach 3: Expression — compute value AND type simultaneously *)
type 'a expr_f =
  | IntLitF of int
  | BoolLitF of bool
  | AddF of 'a * 'a
  | EqF of 'a * 'a    (* equality check *)
  | IfF of 'a * 'a * 'a

let map_ef f = function
  | IntLitF n -> IntLitF n
  | BoolLitF b -> BoolLitF b
  | AddF (a, b) -> AddF (f a, f b)
  | EqF (a, b) -> EqF (f a, f b)
  | IfF (c, t, e) -> IfF (f c, f t, f e)

type fix_expr = FixE of fix_expr expr_f

type value = VInt of int | VBool of bool | VError
type typ = TInt | TBool | TError

let rec mutu_expr val_alg typ_alg (FixE f) =
  let paired = map_ef (mutu_expr val_alg typ_alg) f in
  (val_alg paired, typ_alg paired)

let val_alg : (value * typ) expr_f -> value = function
  | IntLitF n -> VInt n
  | BoolLitF b -> VBool b
  | AddF ((VInt a, _), (VInt b, _)) -> VInt (a + b)
  | EqF ((VInt a, _), (VInt b, _)) -> VBool (a = b)
  | IfF ((VBool true, _), (v, _), _) -> v
  | IfF ((VBool false, _), _, (v, _)) -> v
  | _ -> VError

let typ_alg : (value * typ) expr_f -> typ = function
  | IntLitF _ -> TInt
  | BoolLitF _ -> TBool
  | AddF ((_, TInt), (_, TInt)) -> TInt
  | EqF ((_, TInt), (_, TInt)) -> TBool
  | IfF ((_, TBool), (_, t1), (_, t2)) when t1 = t2 -> t1
  | _ -> TError

let int_lit n = FixE (IntLitF n)
let bool_lit b = FixE (BoolLitF b)
let add_e a b = FixE (AddF (a, b))
let eq_e a b = FixE (EqF (a, b))
let if_e c t e = FixE (IfF (c, t, e))

(* === Tests === *)
let () =
  (* Even/Odd *)
  assert (is_even 0 = true);
  assert (is_even 1 = false);
  assert (is_even 4 = true);
  assert (is_odd 3 = true);
  assert (is_odd 6 = false);

  (* Type-checking expression *)
  let e = add_e (int_lit 1) (int_lit 2) in
  let (v, t) = mutu_expr val_alg typ_alg e in
  assert (v = VInt 3);
  assert (t = TInt);

  let e2 = if_e (eq_e (int_lit 1) (int_lit 1)) (int_lit 42) (int_lit 0) in
  let (v2, t2) = mutu_expr val_alg typ_alg e2 in
  assert (v2 = VInt 42);
  assert (t2 = TInt);

  (* Type error *)
  let e3 = add_e (int_lit 1) (bool_lit true) in
  let (v3, t3) = mutu_expr val_alg typ_alg e3 in
  assert (v3 = VError);
  assert (t3 = TError);

  print_endline "✓ All tests passed"
