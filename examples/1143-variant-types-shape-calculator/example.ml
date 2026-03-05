(* Variant Types — Shape Calculator *)
(* Use variants to model different cases *)

type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
  | Triangle (a, b, c) ->
    let s = (a +. b +. c) /. 2.0 in
    sqrt (s *. (s -. a) *. (s -. b) *. (s -. c))

let perimeter = function
  | Circle r -> 2.0 *. Float.pi *. r
  | Rectangle (w, h) -> 2.0 *. (w +. h)
  | Triangle (a, b, c) -> a +. b +. c

let shapes = [Circle 5.0; Rectangle (3.0, 4.0); Triangle (3.0, 4.0, 5.0)]
let () = List.iter (fun s ->
  Printf.printf "Area: %.2f, Perimeter: %.2f\n" (area s) (perimeter s)
) shapes
