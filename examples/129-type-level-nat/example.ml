(* Example 129: Type-Level Natural Numbers — Peano Arithmetic *)

(* Approach 1: GADT-based Peano numbers *)
type zero = Zero_t
type 'n succ = Succ_t

(* Type-level nat witness *)
type _ nat =
  | Zero : zero nat
  | Succ : 'n nat -> 'n succ nat

let zero = Zero
let one = Succ Zero
let two = Succ (Succ Zero)
let three = Succ (Succ (Succ Zero))

let rec to_int : type n. n nat -> int = function
  | Zero -> 0
  | Succ n -> 1 + to_int n

(* Approach 2: Module-level Peano *)
module type NAT = sig
  type t
  val value : int
end

module Zero_m : NAT = struct type t = zero let value = 0 end

module Succ_m (N : NAT) : NAT = struct
  type t = N.t succ
  let value = N.value + 1
end

module One = Succ_m(Zero_m)
module Two = Succ_m(One)
module Three = Succ_m(Two)

(* Approach 3: Type-safe vectors with length *)
type ('a, 'n) vec =
  | VNil : ('a, zero) vec
  | VCons : 'a * ('a, 'n) vec -> ('a, 'n succ) vec

let v_empty : ('a, zero) vec = VNil
let v_single x : ('a, zero succ) vec = VCons (x, VNil)

let v_head : type a n. (a, n succ) vec -> a = function
  | VCons (x, _) -> x

let v_tail : type a n. (a, n succ) vec -> (a, n) vec = function
  | VCons (_, rest) -> rest

let rec v_length : type a n. (a, n) vec -> int = function
  | VNil -> 0
  | VCons (_, rest) -> 1 + v_length rest

(* Tests *)
let () =
  assert (to_int zero = 0);
  assert (to_int three = 3);
  assert (Zero_m.value = 0);
  assert (Three.value = 3);
  let v = VCons (1, VCons (2, VCons (3, VNil))) in
  assert (v_head v = 1);
  assert (v_length v = 3);
  assert (v_head (v_tail v) = 2);
  Printf.printf "✓ All tests passed\n"
