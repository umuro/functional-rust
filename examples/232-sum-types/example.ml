(* Sum types: one of several alternatives.
   Dual of product types. In category theory: the categorical coproduct. *)

type shape =
  | Circle    of float
  | Rectangle of float * float
  | Triangle  of float * float * float

let area = function
  | Circle r         -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
  | Triangle (a, b, c) ->
    let s = (a +. b +. c) /. 2. in
    sqrt (s *. (s -. a) *. (s -. b) *. (s -. c))

let perimeter = function
  | Circle r         -> 2. *. Float.pi *. r
  | Rectangle (w, h) -> 2. *. (w +. h)
  | Triangle (a, b, c) -> a +. b +. c

let name = function
  | Circle _    -> "circle"
  | Rectangle _ -> "rectangle"
  | Triangle _  -> "triangle"

let () =
  let shapes = [Circle 5.; Rectangle 3. 4.; Triangle 3. 4. 5.] in
  List.iter (fun s ->
    Printf.printf "%s: area=%.2f perimeter=%.2f\n" (name s) (area s) (perimeter s)
  ) shapes
