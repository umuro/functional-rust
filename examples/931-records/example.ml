(* 931: Records — immutable update and pattern matching

   OCaml's `{ r with field = value }` functional-update syntax is the
   direct equivalent of Rust's struct update syntax `Struct { field, ..old }`.
   Both create a new value without mutating the original. *)

(* ── Type definitions ─────────────────────────────────────────────────────── *)

type point = { x : float; y : float }

type rect = {
  origin : point;
  width  : float;
  height : float;
}

(* ── Functions using destructuring and functional update ──────────────────── *)

(* Area via field access — OCaml: let area { width; height; _ } = ... *)
let area { width; height; _ } = width *. height

let perimeter { width; height; _ } = 2.0 *. (width +. height)

(* Functional update: create a new Rect with shifted origin *)
let translate dx dy r =
  { r with origin = { x = r.origin.x +. dx; y = r.origin.y +. dy } }

let contains_point r p =
  p.x >= r.origin.x
  && p.x <= r.origin.x +. r.width
  && p.y >= r.origin.y
  && p.y <= r.origin.y +. r.height

(* Scale: create new rect with scaled dimensions (origin unchanged) *)
let scale factor r =
  { r with width = r.width *. factor; height = r.height *. factor }

(* ── Pattern matching on records ─────────────────────────────────────────── *)

(* Describe a rect shape *)
let describe r =
  match r with
  | { width; height; _ } when width = height ->
    Printf.sprintf "square %.1f×%.1f" width height
  | { width; height; _ } when width > height ->
    Printf.sprintf "landscape %.1f×%.1f" width height
  | { width; height; _ } ->
    Printf.sprintf "portrait %.1f×%.1f" width height

let () =
  let r = { origin = { x = 0.0; y = 0.0 }; width = 10.0; height = 5.0 } in

  assert (abs_float (area r -. 50.0) < Float.epsilon);
  assert (abs_float (perimeter r -. 30.0) < Float.epsilon);

  (* Functional update: translate creates a new record *)
  let r2 = translate 3.0 4.0 r in
  assert (abs_float (r2.origin.x -. 3.0) < Float.epsilon);
  assert (abs_float (r2.origin.y -. 4.0) < Float.epsilon);
  assert (abs_float (r2.width -. 10.0) < Float.epsilon);

  (* Immutability: original unchanged *)
  assert (abs_float (r.origin.x -. 0.0) < Float.epsilon);
  assert (abs_float (r.origin.y -. 0.0) < Float.epsilon);

  (* contains_point *)
  assert (contains_point r { x = 1.0; y = 1.0 });
  assert (not (contains_point r { x = 11.0; y = 1.0 }));
  assert (contains_point r { x = 0.0; y = 0.0 });   (* edge *)
  assert (contains_point r { x = 10.0; y = 5.0 });  (* corner *)

  (* zero-size rect *)
  let r0 = { origin = { x = 5.0; y = 5.0 }; width = 0.0; height = 0.0 } in
  assert (abs_float (area r0) < Float.epsilon);
  assert (contains_point r0 { x = 5.0; y = 5.0 });

  (* describe via pattern matching *)
  let sq = { origin = { x = 0.0; y = 0.0 }; width = 4.0; height = 4.0 } in
  assert (describe sq = "square 4.0×4.0");
  assert (describe r = "landscape 10.0×5.0");

  print_endline "931-records: all tests passed"
