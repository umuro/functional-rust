(* 941: Identity Monad — the simplest possible monad

   Wraps a value with zero extra effects.
   Useful as a base case in monad transformers, and to illustrate the
   monad laws cleanly.

   In OCaml, monads are typically encoded as modules satisfying a MONAD
   signature. Here we show both the module encoding and an inline version. *)

(* ── Module-based Identity monad ─────────────────────────────────────────── *)

module Identity = struct
  type 'a t = Identity of 'a

  (* Monadic return / pure — lift a value *)
  let return x = Identity x

  (* Bind (>>=) — sequence computations *)
  let bind (Identity x) f = f x

  (* Functor map — derived from bind + return *)
  let map f (Identity x) = Identity (f x)

  (* Extract the wrapped value *)
  let run (Identity x) = x
end

(* Infix bind operator *)
let ( >>= ) = Identity.bind

(* Let-binding syntax for monadic chains *)
let ( let* ) = Identity.bind

(* ── Monad laws ───────────────────────────────────────────────────────────── *)

(* left identity:  return a >>= f  ≡  f a
   right identity: m >>= return    ≡  m
   associativity:  (m >>= f) >>= g ≡  m >>= (fun x -> f x >>= g) *)

let () =
  let open Identity in

  (* Functor identity law: map id = id *)
  assert (map Fun.id (return 42) = return 42);

  (* Bind chain *)
  let result =
    let* x = return 10 in
    let* y = return (x * 2) in
    return (y + 1)
  in
  assert (run result = 21);

  (* Left identity: return a >>= f = f a *)
  let f x = return (x * 3) in
  assert ((return 5 >>= f) = f 5);

  (* Right identity: m >>= return = m *)
  let m = return 42 in
  assert ((m >>= return) = m);

  (* Associativity: (m >>= f) >>= g = m >>= (fun x -> f x >>= g) *)
  let m2 = return 2 in
  let g x = return (x + 1) in
  let lhs = (m2 >>= f) >>= g in
  let rhs = m2 >>= (fun x -> f x >>= g) in
  assert (lhs = rhs);

  (* Map composition law: map (f . g) = map f . map g *)
  let f2 x = x + 1 in
  let g2 x = x * 2 in
  let v = return 5 in
  let lhs2 = map (fun x -> f2 (g2 x)) v in
  let rhs2 = map f2 (map g2 v) in
  assert (lhs2 = rhs2);

  (* Practical use: compute in an "identity context" *)
  let computation =
    let* a = return 10 in
    let* b = return 20 in
    let* c = return 30 in
    return (a + b + c)
  in
  assert (run computation = 60);

  print_endline "941-identity-monad: all tests passed"
