(* Tuple Patterns and Destructuring *)
(* Pattern match on tuples and nested structures *)

let distance (x1, y1) (x2, y2) =
  let dx = x2 -. x1 and dy = y2 -. y1 in
  sqrt (dx *. dx +. dy *. dy)

let classify_point = function
  | (0.0, 0.0) -> "origin"
  | (x, 0.0) -> Printf.sprintf "x-axis at %.1f" x
  | (0.0, y) -> Printf.sprintf "y-axis at %.1f" y
  | (x, y) -> Printf.sprintf "(%.1f, %.1f)" x y

let min_max (a, b) = if a <= b then (a, b) else (b, a)

let () =
  Printf.printf "Distance: %.2f\n" (distance (0.0, 0.0) (3.0, 4.0));
  Printf.printf "%s\n" (classify_point (3.0, 0.0));
  let (lo, hi) = min_max (42, 17) in
  Printf.printf "min=%d max=%d\n" lo hi
