(* Pattern Matching: OCaml's most powerful feature *)

(* ── Algebraic data type for shapes ──────────────────────── *)

type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float * float

(* ── Area calculation via pattern matching ────────────────── *)

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
  | Triangle (a, b, c) ->
    let s = (a +. b +. c) /. 2.0 in
    sqrt (s *. (s -. a) *. (s -. b) *. (s -. c))

(* ── Description with guard patterns ─────────────────────── *)

let describe = function
  | Circle r -> Printf.sprintf "Circle with radius %g" r
  | Rectangle (w, h) when Float.equal w h ->
    Printf.sprintf "Square with side %g" w
  | Rectangle (w, h) ->
    Printf.sprintf "Rectangle %g×%g" w h
  | Triangle (a, b, c) when Float.equal a b && Float.equal b c ->
    Printf.sprintf "Equilateral triangle with side %g" a
  | Triangle (a, b, c) ->
    Printf.sprintf "Triangle with sides %g, %g, %g" a b c

(* ── Largest area using fold + Option ────────────────────── *)

let largest_area shapes =
  List.fold_left
    (fun acc s ->
       let a = area s in
       match acc with
       | None -> Some a
       | Some m when a > m -> Some a
       | _ -> acc)
    None shapes

(* ── Count by type ───────────────────────────────────────── *)

let count_by_type shapes =
  List.fold_left
    (fun (c, r, t) s -> match s with
       | Circle _ -> (c + 1, r, t)
       | Rectangle _ -> (c, r + 1, t)
       | Triangle _ -> (c, r, t + 1))
    (0, 0, 0) shapes

(* ── Scale all shapes ────────────────────────────────────── *)

let scale_all factor = List.map (function
  | Circle r -> Circle (r *. factor)
  | Rectangle (w, h) -> Rectangle (w *. factor, h *. factor)
  | Triangle (a, b, c) -> Triangle (a *. factor, b *. factor, c *. factor))

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (abs_float (area (Circle 5.0) -. Float.pi *. 25.0) < 1e-10);
  assert (abs_float (area (Rectangle (3.0, 4.0)) -. 12.0) < 1e-10);
  assert (abs_float (area (Triangle (3.0, 4.0, 5.0)) -. 6.0) < 1e-10);
  assert (describe (Rectangle (5.0, 5.0)) = "Square with side 5");
  assert (largest_area [] = None);
  assert (count_by_type [Circle 1.0; Circle 2.0; Rectangle (1.0, 2.0)] = (2, 1, 0));
  print_endline "✓ All pattern matching tests passed"
