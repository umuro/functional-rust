(* Operator overloading in OCaml via custom infix operators *)
type vec2 = { x: float; y: float }

let ( +^ ) a b = { x = a.x +. b.x; y = a.y +. b.y }
let ( -^ ) a b = { x = a.x -. b.x; y = a.y -. b.y }
let ( *^ ) s v = { x = s *. v.x; y = s *. v.y }
let neg_vec v = { x = -. v.x; y = -. v.y }
let dot a b = a.x *. b.x +. a.y *. b.y
let magnitude v = sqrt (dot v v)
let string_of_vec {x; y} = Printf.sprintf "Vec2(%.2f, %.2f)" x y

let () =
  let a = {x = 3.0; y = 4.0} in
  let b = {x = 1.0; y = 2.0} in
  Printf.printf "a + b = %s\n" (string_of_vec (a +^ b));
  Printf.printf "a - b = %s\n" (string_of_vec (a -^ b));
  Printf.printf "2 * a = %s\n" (string_of_vec (2.0 *^ a));
  Printf.printf "|a| = %.2f\n" (magnitude a)
