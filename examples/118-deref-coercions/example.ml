(* Example 118: Deref Coercions *)

(* OCaml doesn't have deref coercions — types don't automatically convert.
   But we can show analogous patterns. *)

(* Approach 1: String and bytes — no auto-conversion *)
let approach1 () =
  let s = "hello" in
  let bytes = Bytes.of_string s in
  (* Must explicitly convert back *)
  let s2 = Bytes.to_string bytes in
  assert (s = s2);
  Printf.printf "String: %s, Bytes length: %d\n" s (Bytes.length bytes)

(* Approach 2: Coercion via :> for subtypes *)
type animal = [`Dog | `Cat | `Bird]
type pet = [`Dog | `Cat]

let describe_animal : animal -> string = function
  | `Dog -> "dog"
  | `Cat -> "cat"
  | `Bird -> "bird"

let approach2 () =
  let my_pet : pet = `Dog in
  let description = describe_animal (my_pet :> animal) in
  assert (description = "dog");
  Printf.printf "Pet: %s\n" description

(* Approach 3: Explicit wrapping/unwrapping *)
module Wrapper : sig
  type t
  val create : int -> t
  val get : t -> int
  val map : (int -> int) -> t -> t
end = struct
  type t = int
  let create x = x
  let get x = x
  let map f x = f x
end

let approach3 () =
  let w = Wrapper.create 42 in
  let v = Wrapper.get w in
  let w2 = Wrapper.map (fun x -> x + 1) w in
  assert (v = 42);
  assert (Wrapper.get w2 = 43);
  Printf.printf "Wrapped: %d, Mapped: %d\n" v (Wrapper.get w2)

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
