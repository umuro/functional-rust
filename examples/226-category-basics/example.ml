(* 226: Category Basics
   A category has: objects, morphisms, identity, and composition.
   In OCaml: types = objects, functions = morphisms. *)

(* ── Approach 1: Simple compose and identity ──────────────────────────────── *)

let identity x = x

(* compose f g x = f (g x) — standard function composition *)
let compose f g x = f (g x)

(* Infix operator for readability *)
let ( << ) f g = compose f g

(* ── Category laws ────────────────────────────────────────────────────────── *)

(* Left identity:  (identity << f) x = f x  *)
(* Right identity: (f << identity) x = f x  *)
(* Associativity:  ((f << g) << h) x = (f << (g << h)) x *)

(* ── Approach 2: Category as a first-class module (type class encoding) ───── *)

module type CATEGORY = sig
  (* A morphism from 'a to 'b *)
  type ('a, 'b) hom
  val id      : ('a, 'a) hom
  val compose : ('b, 'c) hom -> ('a, 'b) hom -> ('a, 'c) hom
end

(* The function category: morphisms are ordinary functions *)
module FnCategory : CATEGORY with type ('a, 'b) hom = 'a -> 'b = struct
  type ('a, 'b) hom = 'a -> 'b
  let id      = fun x -> x
  let compose f g = fun x -> f (g x)
end

(* ── Approach 3: Kleisli category — morphisms of type a -> b option ─────── *)

(* In the Kleisli category for Option:
   - identity = fun a -> Some a
   - composition uses Option.bind (>>=) *)

let kleisli_id a = Some a

(* Kleisli composition: f >=> g  means  fun a -> g a >>= f *)
let kleisli_compose f g a =
  match g a with
  | None   -> None
  | Some b -> f b

let ( >=> ) f g = kleisli_compose f g

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  let add1 = fun x -> x + 1 in
  let mul2 = fun x -> x * 2 in
  let sub3 = fun x -> x - 3 in

  (* Composition *)
  Printf.printf "compose(add1, mul2)(5)  = %d\n" ((compose add1 mul2) 5);
  Printf.printf "(add1 << mul2)(5)       = %d\n" ((add1 << mul2) 5);

  (* Left/right identity law *)
  Printf.printf "identity law left:  %b\n" ((identity << add1) 10 = add1 10);
  Printf.printf "identity law right: %b\n" ((add1 << identity) 10 = add1 10);

  (* Associativity law *)
  let lhs = ((add1 << mul2) << sub3) 7 in
  let rhs = (add1 << (mul2 << sub3)) 7 in
  Printf.printf "associativity:      %b  (%d = %d)\n" (lhs = rhs) lhs rhs;

  (* FnCategory module *)
  let open FnCategory in
  let f = compose add1 mul2 in
  Printf.printf "FnCategory compose: %d\n" (f 5);
  Printf.printf "FnCategory id:      %d\n" (id 42);

  (* Kleisli category *)
  let safe_div y x = if x = 0 then None else Some (y / x) in
  let safe_sqrt x  = if x < 0  then None else Some (int_of_float (sqrt (float_of_int x))) in
  let pipeline = safe_sqrt >=> (safe_div 100) in
  Printf.printf "kleisli 4   -> %s\n" (Option.fold ~none:"None" ~some:string_of_int (pipeline 4));
  Printf.printf "kleisli 0   -> %s\n" (Option.fold ~none:"None" ~some:string_of_int (pipeline 0));
  Printf.printf "kleisli -1  -> %s\n" (Option.fold ~none:"None" ~some:string_of_int (pipeline (-1)));
  Printf.printf "kleisli_id: %b\n"
    (kleisli_id 5 = Some 5)
