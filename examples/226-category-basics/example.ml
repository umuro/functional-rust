(* Example 226: Category Basics *)
(* A category has objects, morphisms, composition, and identity *)

(* === Approach 1: Category as composition and identity === *)

(* In OCaml, types are objects and functions are morphisms.
   Composition is (.) and identity is Fun.id *)

let compose f g x = f (g x)
let id x = x

(* Category laws:
   1. Left identity:  compose id f = f
   2. Right identity: compose f id = f
   3. Associativity:  compose (compose f g) h = compose f (compose g h) *)

let () =
  let f x = x + 1 in
  let g x = x * 2 in
  let h x = x - 3 in
  (* Left identity *)
  assert (compose id f 5 = f 5);
  (* Right identity *)
  assert (compose f id 5 = f 5);
  (* Associativity *)
  assert (compose (compose f g) h 10 = compose f (compose g h) 10);
  Printf.printf "Approach 1 - Category laws verified\n"

(* === Approach 2: Explicit Category module type === *)

module type CATEGORY = sig
  type ('a, 'b) morphism
  val id : ('a, 'a) morphism
  val compose : ('b, 'c) morphism -> ('a, 'b) morphism -> ('a, 'c) morphism
end

(* The category of OCaml types and functions *)
module FnCategory : CATEGORY with type ('a, 'b) morphism = 'a -> 'b = struct
  type ('a, 'b) morphism = 'a -> 'b
  let id x = x
  let compose f g x = f (g x)
end

let () =
  let open FnCategory in
  let f = fun x -> x + 1 in
  let g = fun x -> x * 2 in
  let result = compose f g 5 in
  assert (result = 11);
  assert (compose id f 5 = f 5);
  Printf.printf "Approach 2 - FnCategory works: compose (+1) (*2) 5 = %d\n" result

(* === Approach 3: Kleisli category (morphisms a -> 'b option) === *)

module KleisliOption = struct
  type ('a, 'b) morphism = 'a -> 'b option
  let id x = Some x
  let compose f g x =
    match g x with
    | None -> None
    | Some y -> f y
end

let () =
  let open KleisliOption in
  let safe_div x = if x = 0 then None else Some (100 / x) in
  let safe_succ x = Some (x + 1) in
  assert (compose safe_succ safe_div 5 = Some 21);
  assert (compose safe_succ safe_div 0 = None);
  (* Identity law *)
  assert (compose id safe_div 5 = safe_div 5);
  assert (compose safe_div id 5 = safe_div 5);
  Printf.printf "Approach 3 - Kleisli category verified\n"

let () = Printf.printf "✓ All tests passed\n"
