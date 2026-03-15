(* 734: Typestate basics — compile-time state machine in OCaml *)
(* Rust uses PhantomData<State> to encode state in the type parameter.
   OCaml uses phantom types: type parameters that appear in the type
   but not in the runtime representation.

   Transitions consume the old state and return a value with a new type.
   Invalid transitions simply do not exist as functions — they cannot be
   called, so they cannot be written. This is a compile-time guarantee. *)

(* ── State marker types (abstract, no values) ────────────────────────────── *)
(* These types are only used as type parameters; they have no constructors. *)

type red    = private Red_
type green  = private Green_
type yellow = private Yellow_

(* ── Traffic Light ─────────────────────────────────────────────────────── *)
(* 'state is the phantom type parameter encoding the current state.
   At runtime, a light is just unit — no space overhead. *)

type 'state light = Light

(* Create a light — must start Red. Returns a Light in Red state. *)
let new_light () : red light =
  print_endline "Light: Red";
  Light

(* Red → Green: only callable on (red light) *)
let go (Light : red light) : green light =
  print_endline "Light: Green";
  Light

(* Green → Yellow: only callable on (green light) *)
let slow (Light : green light) : yellow light =
  print_endline "Light: Yellow";
  Light

(* Yellow → Red: only callable on (yellow light) *)
let stop (Light : yellow light) : red light =
  print_endline "Light: Red";
  Light

(* Size: a light is represented as unit — zero bytes of payload *)

let () =
  (* Full cycle — each step only type-checks if the previous step was valid *)
  let red    = new_light () in
  let green  = go red in
  let yellow = slow green in
  let _red2  = stop yellow in
  print_endline "full cycle: ok";

  (* The following would be a COMPILE ERROR (type mismatch):
       let _ = slow red    (* red light has no slow *)
       let _ = go yellow   (* yellow light has no go *)
  *)

  (* Size check: light is represented as a unit constructor — 0 payload bytes *)
  assert (Obj.size (Obj.repr Light) = 0 ||
          Obj.tag  (Obj.repr Light) = Obj.int_tag);
  print_endline "light is zero-payload: ok";

  print_endline "All assertions passed."
