(* OCaml doesn't have sealed traits natively, but we can simulate them
   with module types + private constructors. The pattern restricts who
   can create instances of a type. *)

(* Shape hierarchy: only shapes defined here can implement Shape *)
module type SHAPE = sig
  type t
  val area     : t -> float
  val perimeter: t -> float
  val name     : string
end

module Circle : SHAPE = struct
  type t = { radius: float }
  let area     c = Float.pi *. c.radius *. c.radius
  let perimeter c = 2. *. Float.pi *. c.radius
  let name = "circle"
end

module Rect : SHAPE = struct
  type t = { w: float; h: float }
  let area     r = r.w *. r.h
  let perimeter r = 2. *. (r.w +. r.h)
  let name = "rectangle"
end

(* Existential wrapper for dispatch *)
type shape = Shape : (module SHAPE with type t = 'a) * 'a -> shape

let circle r = Shape ((module Circle), { Circle.radius = r })
let rect w h = Shape ((module Rect),   { Rect.w = w; Rect.h = h })

let area (Shape ((module S), s)) = S.area s
let perimeter (Shape ((module S), s)) = S.perimeter s
let name (Shape ((module S), _)) = S.name

let () =
  let shapes = [circle 5.; rect 3. 4.; circle 1.] in
  List.iter (fun s ->
    Printf.printf "%s: area=%.2f perimeter=%.2f\n"
      (name s) (area s) (perimeter s)
  ) shapes
