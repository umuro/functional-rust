(* Derive macro concepts in OCaml *)
(* OCaml show and eq from ppx_deriving *)

(* Without ppx: manual implementations *)
type point = { x: float; y: float }

(* Manual "derive Debug" *)
let show_point {x; y} = Printf.sprintf "Point { x = %g; y = %g }" x y

(* Manual "derive Eq" *)
let equal_point a b = a.x = b.x && a.y = b.y

(* Manual "derive Ord" *)
let compare_point a b =
  let cx = compare a.x b.x in
  if cx <> 0 then cx else compare a.y b.y

(* With ppx_deriving, you'd write: *)
(* type point = { x: float; y: float } [@@deriving show, eq, ord] *)

type shape = Circle of float | Rectangle of float * float | Triangle of float * float * float

let show_shape = function
  | Circle r -> Printf.sprintf "Circle(%g)" r
  | Rectangle (w, h) -> Printf.sprintf "Rectangle(%g, %g)" w h
  | Triangle (a, b, c) -> Printf.sprintf "Triangle(%g, %g, %g)" a b c

let () =
  let p1 = {x = 1.0; y = 2.0} in
  let p2 = {x = 1.0; y = 2.0} in
  let p3 = {x = 3.0; y = 4.0} in
  Printf.printf "%s\n" (show_point p1);
  Printf.printf "p1 = p2: %b\n" (equal_point p1 p2);
  Printf.printf "p1 = p3: %b\n" (equal_point p1 p3);
  Printf.printf "compare: %d\n" (compare_point p1 p3);
  List.iter (fun s -> Printf.printf "%s\n" (show_shape s))
    [Circle 5.0; Rectangle 3.0 4.0; Triangle 3.0 4.0 5.0]
