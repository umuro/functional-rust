(* Example 212: Van Laarhoven Lenses *)

(* The Van Laarhoven encoding: a lens is a polymorphic function
   that works for ANY functor f:
   
   type ('s,'t,'a,'b) lens = forall f. Functor f => (a -> f b) -> s -> f t
   
   This single type unifies get, set, and modify! *)

(* Approach 1: Module-based Van Laarhoven in OCaml *)

module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

(* Identity functor — used for "modify" *)
module Identity = struct
  type 'a t = 'a
  let map f x = f x
  let run x = x
end

(* Const functor — used for "get" *)
module Const (M : sig type t end) = struct
  type 'a t = M.t
  let map _f x = x
  let run x = x
end

(* A Van Laarhoven lens for a specific functor *)
module type VL_LENS = sig
  type s
  type a
  val lens : (module FUNCTOR with type 'x t = 'x) -> (a -> a) -> s -> s
  (* We can't truly express rank-2 types in OCaml, so we use modules *)
end

(* Approach 2: Practical encoding using two functions *)
(* In practice, OCaml can't do rank-2 polymorphism directly,
   so we encode VL lenses as a record with "for each functor" *)

type ('s, 'a) vl_lens = {
  run_identity : ('a -> 'a) -> 's -> 's;         (* over/modify *)
  run_const    : 's -> 'a;                         (* get *)
}

let vl_view l s = l.run_const s
let vl_over l f s = l.run_identity f s
let vl_set l a s = vl_over l (fun _ -> a) s

(* Create VL lenses for record fields *)
type person = { name : string; age : int }

let vl_name : (person, string) vl_lens = {
  run_identity = (fun f p -> { p with name = f p.name });
  run_const = (fun p -> p.name);
}

let vl_age : (person, int) vl_lens = {
  run_identity = (fun f p -> { p with age = f p.age });
  run_const = (fun p -> p.age);
}

(* Approach 3: Composition of VL lenses *)
let vl_compose (outer : ('s, 'a) vl_lens) (inner : ('a, 'b) vl_lens) : ('s, 'b) vl_lens = {
  run_identity = (fun f s -> outer.run_identity (inner.run_identity f) s);
  run_const = (fun s -> inner.run_const (outer.run_const s));
}

type address = { street : string; city : string }
type employee = { emp_name : string; addr : address }

let vl_addr : (employee, address) vl_lens = {
  run_identity = (fun f e -> { e with addr = f e.addr });
  run_const = (fun e -> e.addr);
}

let vl_city : (address, string) vl_lens = {
  run_identity = (fun f a -> { a with city = f a.city });
  run_const = (fun a -> a.city);
}

let vl_emp_city = vl_compose vl_addr vl_city

(* === Tests === *)
let () =
  let alice = { name = "Alice"; age = 30 } in

  (* VL lens get *)
  assert (vl_view vl_name alice = "Alice");
  assert (vl_view vl_age alice = 30);

  (* VL lens set *)
  let alice2 = vl_set vl_name "Alicia" alice in
  assert (vl_view vl_name alice2 = "Alicia");

  (* VL lens over *)
  let alice3 = vl_over vl_age (fun a -> a + 1) alice in
  assert (vl_view vl_age alice3 = 31);

  (* Composed VL lens *)
  let emp = { emp_name = "Bob"; addr = { street = "123 Main"; city = "NYC" } } in
  assert (vl_view vl_emp_city emp = "NYC");
  let emp2 = vl_set vl_emp_city "LA" emp in
  assert (vl_view vl_emp_city emp2 = "LA");
  let emp3 = vl_over vl_emp_city String.uppercase_ascii emp in
  assert (vl_view vl_emp_city emp3 = "NYC");

  (* Composition is just function composition *)
  let composed_over = vl_over (vl_compose vl_addr vl_city) String.uppercase_ascii emp in
  assert (vl_view vl_emp_city composed_over = "NYC");

  print_endline "✓ All tests passed"
