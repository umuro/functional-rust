(* Pattern Matching in OCaml *)

(* Basic patterns *)
type color = Red | Green | Blue | RGB of int * int * int

let describe_color = function
  | Red -> "pure red"
  | Green -> "pure green"  
  | Blue -> "pure blue"
  | RGB (r, g, b) -> Printf.sprintf "RGB(%d, %d, %d)" r g b

(* Guards *)
let classify_number n =
  match n with
  | x when x < 0 -> "negative"
  | 0 -> "zero"
  | x when x > 0 && x <= 10 -> "small positive"
  | _ -> "large positive"

(* Nested patterns *)
type shape =
  | Circle of float
  | Rectangle of float * float
  | Point of float * float

let area = function
  | Circle r -> 3.14159 *. r *. r
  | Rectangle (w, h) -> w *. h
  | Point _ -> 0.0

let describe_shape = function
  | Circle r when r > 10.0 -> "large circle"
  | Circle _ -> "small circle"
  | Rectangle (w, h) when w = h -> "square"
  | Rectangle _ -> "rectangle"
  | Point (0.0, 0.0) -> "origin"
  | Point _ -> "point"

(* Tuple and list patterns *)
let swap (x, y) = (y, x)

let first_two = function
  | [] -> None
  | [_] -> None
  | x :: y :: _ -> Some (x, y)

(* As patterns *)
let duplicate_first = function
  | [] -> []
  | (x :: _) as lst -> x :: lst

(* Or patterns *)
let is_primary_color = function
  | Red | Green | Blue -> true
  | RGB _ -> false

(* Examples *)
let () =
  Printf.printf "Red: %s\n" (describe_color Red);
  Printf.printf "RGB: %s\n" (describe_color (RGB (255, 128, 0)));
  
  Printf.printf "Classify -5: %s\n" (classify_number (-5));
  Printf.printf "Classify 7: %s\n" (classify_number 7);
  
  let circle = Circle 15.0 in
  Printf.printf "Area: %.2f\n" (area circle);
  Printf.printf "Shape: %s\n" (describe_shape circle);
  
  let rect = Rectangle (5.0, 5.0) in
  Printf.printf "Square: %s\n" (describe_shape rect);
  
  Printf.printf "Swap (1,2): %s\n" 
    (let (a, b) = swap (1, 2) in Printf.sprintf "(%d,%d)" a b);
  
  Printf.printf "Is Red primary? %b\n" (is_primary_color Red);
  Printf.printf "Is RGB primary? %b\n" (is_primary_color (RGB (100, 100, 100)))
