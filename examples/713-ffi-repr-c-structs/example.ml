(* OCaml: struct layout for C interop via Ctypes (conceptual). *)

(* Equivalent C layout: struct Point2D { double x; double y; }; *)
type point2d = { x : float; y : float }

(* Equivalent C: struct Rect { Point2D origin; double width; double height; }; *)
type rect = { origin : point2d; width : float; height : float }

let area (r : rect) : float = r.width *. r.height
let perimeter (r : rect) : float = 2.0 *. (r.width +. r.height)

let () =
  let r = { origin = { x = 1.0; y = 2.0 }; width = 10.0; height = 5.0 } in
  Printf.printf "Area:      %.1f\n" (area r);
  Printf.printf "Perimeter: %.1f\n" (perimeter r)
