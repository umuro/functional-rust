(* Type Annotations and Constraints *)
(* Explicit type annotations in OCaml *)

(* Parameter annotations *)
let add (x : int) (y : int) : int = x + y

(* Return type annotation *)
let divide (x : float) (y : float) : float option =
  if y = 0.0 then None else Some (x /. y)

(* Polymorphic annotation *)
let first (pair : 'a * 'b) : 'a = fst pair
let swap (x : 'a) (y : 'b) : 'b * 'a = (y, x)

(* Type alias *)
type point = float * float
type vector = float * float

let translate ((px, py) : point) ((vx, vy) : vector) : point =
  (px +. vx, py +. vy)

let () =
  let p = translate (1.0, 2.0) (3.0, 4.0) in
  Printf.printf "(%.1f, %.1f)\n" (fst p) (snd p)
