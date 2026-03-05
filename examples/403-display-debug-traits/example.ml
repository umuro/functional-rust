(* Display and Debug formatting in OCaml *)

type color = Red | Green | Blue | Rgb of int * int * int

let string_of_color = function
  | Red -> "red"
  | Green -> "green"
  | Blue -> "blue"
  | Rgb (r, g, b) -> Printf.sprintf "rgb(%d,%d,%d)" r g b

let debug_color = function
  | Red -> "Color::Red"
  | Green -> "Color::Green"
  | Blue -> "Color::Blue"
  | Rgb (r, g, b) -> Printf.sprintf "Color::Rgb(%d, %d, %d)" r g b

type point = { x: float; y: float }

let string_of_point {x; y} = Printf.sprintf "(%.2f, %.2f)" x y
let debug_point {x; y} = Printf.sprintf "Point { x: %g, y: %g }" x y

let () =
  let colors = [Red; Green; Rgb (128, 64, 32)] in
  List.iter (fun c ->
    Printf.printf "display: %s  debug: %s\n"
      (string_of_color c) (debug_color c)
  ) colors;
  let p = {x = 3.14; y = 2.72} in
  Printf.printf "display: %s  debug: %s\n" (string_of_point p) (debug_point p)
