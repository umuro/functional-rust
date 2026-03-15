(* Example 124: Dynamic Dispatch — dyn Trait vs impl Trait *)

(* OCaml uses virtual dispatch for objects, first-class modules for
   trait-like polymorphism. *)

(* Approach 1: First-class modules — like dyn Trait *)
module type Shape = sig
  val area : unit -> float
  val name : unit -> string
end

let total_area (shapes : (module Shape) list) =
  List.fold_left (fun acc (module S : Shape) -> acc +. S.area ()) 0.0 shapes

let circle r : (module Shape) = (module struct
  let area () = Float.pi *. r *. r
  let name () = "circle"
end)

let rect w h : (module Shape) = (module struct
  let area () = w *. h
  let name () = "rectangle"
end)

let approach1 () =
  let shapes = [circle 5.0; rect 3.0 4.0; circle 2.0] in
  let total = total_area shapes in
  Printf.printf "Total area: %.2f\n" total;
  assert (total > 90.0)

(* Approach 2: Object-oriented style *)
class virtual shape_obj = object
  method virtual area : float
  method virtual name : string
end

class circle_obj r = object
  inherit shape_obj
  method area = Float.pi *. r *. r
  method name = "circle"
end

class rect_obj w h = object
  inherit shape_obj
  method area = w *. h
  method name = "rectangle"
end

let approach2 () =
  let shapes = [new circle_obj 5.0; new rect_obj 3.0 4.0] in
  let total = List.fold_left (fun acc s -> acc +. s#area) 0.0 shapes in
  Printf.printf "OO total: %.2f\n" total

(* Approach 3: Variant-based dispatch — monomorphic *)
type shape_v = Circle of float | Rect of float * float

let area_v = function
  | Circle r -> Float.pi *. r *. r
  | Rect (w, h) -> w *. h

let approach3 () =
  let shapes = [Circle 5.0; Rect (3.0, 4.0)] in
  let total = List.fold_left (fun acc s -> acc +. area_v s) 0.0 shapes in
  Printf.printf "Variant total: %.2f\n" total

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
