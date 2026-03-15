(* 102: Clone vs Copy
   OCaml uses a GC — there is no ownership. All values are freely shareable.
   "Copy" types (ints, floats, chars, tuples of those) are value-copied on binding.
   "Clone" (heap types like strings, lists) are shared by reference — no explicit copy needed.
   Explicit deep copy: String.copy (deprecated) or manually reconstructing a value.

   The key distinction OCaml cares about:
   - Structural equality (=) vs physical equality (==)
   - Mutable records/arrays do need explicit copying to avoid aliasing *)

type point = { x: float; y: float }  (* effectively Copy — all fields are floats *)
type person = { name: string; age: int }  (* fields include a heap string *)

(* In OCaml, point is passed by value — assigning creates an independent copy *)
let demonstrate_copy () =
  let p1 = { x = 1.0; y = 2.0 } in
  let p2 = p1 in  (* structural copy — p1 and p2 are independent *)
  assert (p1 = p2);
  assert (p1.x = p2.x)

(* Strings are immutable in OCaml — sharing is safe *)
let demonstrate_string_sharing () =
  let s1 = "hello" in
  let s2 = s1 in      (* same string object — safe, immutable *)
  let s3 = String.concat "" [s1]  (* explicit "clone" via new string *) in
  assert (s1 = s2);
  assert (s1 = s3)

(* Lists: persistent data structures — safe to share *)
let demonstrate_list () =
  let v1 = [1; 2; 3] in
  let v2 = v1 in       (* sharing the same list — safe *)
  assert (v1 = v2);
  let v3 = List.map (fun x -> x) v1  (* explicit clone — new list *) in
  assert (v1 = v3)

(* Mutable records need explicit copy to avoid aliasing *)
type mutable_counter = { mutable count: int }

let demonstrate_mutable () =
  let c1 = { count = 0 } in
  let c2 = { c1 with count = c1.count } in  (* explicit copy *)
  c1.count <- 5;
  assert (c1.count = 5);
  assert (c2.count = 0)  (* c2 is independent *)

let () =
  let a = 42 in
  let b = a in
  assert (a = b);

  demonstrate_copy ();
  demonstrate_string_sharing ();
  demonstrate_list ();
  demonstrate_mutable ();

  Printf.printf "Clone/Copy concept: OCaml has GC sharing by default\n"
