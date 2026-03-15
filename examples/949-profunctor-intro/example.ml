(* 949: Profunctor Intro

   A profunctor p a b is:
     - contravariant in a (input): lmap / contramap
     - covariant in b (output): rmap / map
     - dimap combines both: dimap f g p = rmap g (lmap f p) = g ∘ p ∘ f

   Functions `a -> b` are the canonical profunctor example.
   OCaml's module system lets us encode this cleanly with a PROFUNCTOR signature. *)

(* ── Module type ─────────────────────────────────────────────────────────── *)

module type PROFUNCTOR = sig
  type ('a, 'b) t
  val dimap : ('c -> 'a) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
  val lmap  : ('c -> 'a) -> ('a, 'b) t -> ('c, 'b) t
  val rmap  : ('b -> 'd) -> ('a, 'b) t -> ('a, 'd) t
end

(* ── Concrete Mapper: just wraps a function ──────────────────────────────── *)

module Mapper : sig
  type ('a, 'b) t
  val make  : ('a -> 'b) -> ('a, 'b) t
  val apply : ('a, 'b) t -> 'a -> 'b
  val dimap : ('c -> 'a) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
  val lmap  : ('c -> 'a) -> ('a, 'b) t -> ('c, 'b) t
  val rmap  : ('b -> 'd) -> ('a, 'b) t -> ('a, 'd) t
end = struct
  type ('a, 'b) t = { f : 'a -> 'b }

  let make f = { f }
  let apply m a = m.f a

  (* dimap pre post p = post ∘ p ∘ pre *)
  let dimap pre post m = { f = fun c -> post (m.f (pre c)) }

  (* lmap: adapt only the input — dimap f id *)
  let lmap pre m = { f = fun c -> m.f (pre c) }

  (* rmap: adapt only the output — dimap id g *)
  let rmap post m = { f = fun a -> post (m.f a) }
end

(* ── Star: Mapper lifted into Option context ─────────────────────────────── *)

(* Star f a b = a -> f b (here f = Option) *)
module Star : sig
  type ('a, 'b) t
  val make  : ('a -> 'b option) -> ('a, 'b) t
  val apply : ('a, 'b) t -> 'a -> 'b option
  val lmap  : ('c -> 'a) -> ('a, 'b) t -> ('c, 'b) t
  val rmap  : ('b -> 'd) -> ('a, 'b) t -> ('a, 'd) t
end = struct
  type ('a, 'b) t = { run : 'a -> 'b option }
  let make f = { run = f }
  let apply s a = s.run a
  let lmap pre s = { run = fun c -> s.run (pre c) }
  let rmap post s = { run = fun a -> Option.map post (s.run a) }
end

(* ── Costar: dual of Star — result wrapped in Option on input side ─────── *)

(* Demonstrates contravariance: Costar f a b = f a -> b *)
module Costar : sig
  type ('a, 'b) t
  val make   : ('a option -> 'b) -> ('a, 'b) t
  val apply  : ('a, 'b) t -> 'a option -> 'b
  val lmap   : ('c -> 'a) -> ('a, 'b) t -> ('c, 'b) t
  val rmap   : ('b -> 'd) -> ('a, 'b) t -> ('a, 'd) t
end = struct
  type ('a, 'b) t = { run : 'a option -> 'b }
  let make f = { run = f }
  let apply cs a = cs.run a
  let lmap pre cs = { run = fun c -> cs.run (Option.map pre c) }
  let rmap post cs = { run = fun a -> post (cs.run a) }
end

let () =
  let open Mapper in

  (* lmap: adapt input — 42 -> "42" -> length = 2 *)
  let m1 = make String.length |> lmap string_of_int in
  assert (apply m1 42 = 2);

  (* rmap: adapt output — uppercase then length *)
  let m2 = make String.uppercase_ascii |> rmap String.length in
  assert (apply m2 "hello" = 5);

  (* dimap: adapt both — int -> string -> uppercase -> length *)
  let m3 = make String.uppercase_ascii
           |> dimap string_of_int String.length in
  assert (apply m3 7 = 1);   (* "7" -> "7" -> 1 *)

  (* Profunctor identity law: dimap id id p = p *)
  let p1 = make (fun x -> x * 2) in
  let p2 = dimap (fun x -> x) (fun x -> x) p1 in
  assert (apply p1 21 = apply p2 21);

  (* Profunctor composition law:
     dimap (f . g) (h . k) = dimap g h . dimap f k *)
  let f x = x + 1 and g x = x * 2 in
  let h s = s ^ "!" and k s = String.uppercase_ascii s in
  let base = make (fun n -> string_of_int n) in
  let lhs  = dimap (fun x -> f (g x)) (fun s -> h (k s)) base in
  let rhs  = (dimap g k base) |> (dimap f h) in
  assert (apply lhs 3 = apply rhs 3);  (* both: 3*2=6, 6+1=7, "7" -> "7!" *)

  (* Star: parse int then add 10 *)
  let open Star in
  let parse = make (fun s -> int_of_string_opt s) |> rmap (fun n -> n + 10) in
  assert (apply parse "5"   = Some 15);
  assert (apply parse "bad" = None);

  (* Star lmap: pre-process input *)
  let parse2 = make (fun s -> int_of_string_opt s)
               |> lmap (fun (s, _extra) -> s) in
  assert (apply parse2 ("42", "ignored") = Some 42);

  print_endline "949-profunctor-intro: all tests passed"
