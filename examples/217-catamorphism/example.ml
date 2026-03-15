(* 217: Catamorphism — The Universal Fold
   A catamorphism (cata) is a single function that encodes all bottom-up
   traversals of a recursive structure. Write one non-recursive algebra;
   cata handles all the recursion. *)

(* ── Step 1: Base functor — expression shape with children abstracted ──────── *)

(* ExprF 'a: one layer of an arithmetic expression.
   'a is the type for child positions; LitF has no children. *)
type 'a expr_f =
  | LitF    of int
  | AddF    of 'a * 'a
  | MulF    of 'a * 'a
  | NegF    of 'a
  | IfZeroF of 'a * 'a * 'a   (* cond, then, else *)

(* Functorial map: apply f to every child position *)
let fmap (f : 'a -> 'b) : 'a expr_f -> 'b expr_f = function
  | LitF n           -> LitF n
  | AddF (a, b)      -> AddF (f a, f b)
  | MulF (a, b)      -> MulF (f a, f b)
  | NegF a           -> NegF (f a)
  | IfZeroF (c, t, e) -> IfZeroF (f c, f t, f e)

(* ── Step 2: Fix wrapper — ties the recursive knot ──────────────────────── *)

(* 'fix' wraps ExprF<fix> to form a fully recursive tree *)
type fix = Fix of fix expr_f

let wrap layer = Fix layer
let unfix (Fix layer) = layer

(* Smart constructors *)
let lit n         = wrap (LitF n)
let mk_add a b    = wrap (AddF (a, b))
let mk_mul a b    = wrap (MulF (a, b))
let mk_neg a      = wrap (NegF a)
let if_zero c t e = wrap (IfZeroF (c, t, e))

(* ── Step 3: cata — the one and only recursive function ─────────────────── *)

(* Catamorphism: fold bottom-up using algebra alg.
   alg handles one node after all children have been reduced.
   cata alg (Fix layer) = alg (fmap (cata alg) layer) *)
let rec cata (alg : 'a expr_f -> 'a) (Fix layer) : 'a =
  alg (fmap (cata alg) layer)

(* ── Algebras — zero recursion, one concern each ────────────────────────── *)

(* Evaluate to an integer *)
let run_eval expr =
  cata (function
    | LitF n           -> n
    | AddF (a, b)      -> a + b
    | MulF (a, b)      -> a * b
    | NegF a           -> -a
    | IfZeroF (c, t, e) -> if c = 0 then t else e
  ) expr

(* Pretty-print as a string *)
let show expr =
  cata (function
    | LitF n           -> string_of_int n
    | AddF (a, b)      -> Printf.sprintf "(%s + %s)" a b
    | MulF (a, b)      -> Printf.sprintf "(%s * %s)" a b
    | NegF a           -> Printf.sprintf "(-%s)" a
    | IfZeroF (c, t, e) -> Printf.sprintf "(ifz %s then %s else %s)" c t e
  ) expr

(* Count total nodes in the tree *)
let count_nodes expr =
  cata (function
    | LitF _           -> 1
    | AddF (a, b)      -> 1 + a + b
    | MulF (a, b)      -> 1 + a + b
    | NegF a           -> 1 + a
    | IfZeroF (c, t, e) -> 1 + c + t + e
  ) expr

(* Collect all literal values in left-to-right order *)
let collect_lits expr =
  cata (function
    | LitF n           -> [n]
    | AddF (a, b)      -> a @ b
    | MulF (a, b)      -> a @ b
    | NegF a           -> a
    | IfZeroF (c, t, e) -> c @ t @ e
  ) expr

(* Maximum depth of the tree *)
let depth expr =
  cata (function
    | LitF _           -> 0
    | AddF (a, b)      -> 1 + max a b
    | MulF (a, b)      -> 1 + max a b
    | NegF a           -> 1 + a
    | IfZeroF (c, t, e) -> 1 + max c (max t e)
  ) expr

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* (2 + 3) * (-4)  →  -20 *)
  let sample = mk_mul (mk_add (lit 2) (lit 3)) (mk_neg (lit 4)) in
  Printf.printf "show:        %s\n"  (show sample);
  Printf.printf "evaluate:    %d\n"  (run_eval sample);
  Printf.printf "nodes:       %d\n"  (count_nodes sample);
  Printf.printf "lits:        [%s]\n"
    (collect_lits sample |> List.map string_of_int |> String.concat ";");
  Printf.printf "depth:       %d\n"  (depth sample);

  (* ifz 0 then 10 else 99  →  10 *)
  let iz = if_zero (lit 0) (lit 10) (lit 99) in
  Printf.printf "\nifz example: %s = %d\n" (show iz) (run_eval iz);

  (* Custom algebra inline: count negative literals *)
  let tree = mk_add (mk_neg (lit (-3))) (mk_mul (lit 5) (mk_neg (lit (-1)))) in
  let neg_count = cata (function
    | LitF n           -> if n < 0 then 1 else 0
    | AddF (a, b)      -> a + b
    | MulF (a, b)      -> a + b
    | NegF a           -> a
    | IfZeroF (c, t, e) -> c + t + e
  ) tree in
  Printf.printf "\nnegative literals in tree: %d\n" neg_count
