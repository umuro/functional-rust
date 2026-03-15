(* Example 076: Trait Objects — Dynamic Dispatch *)
(* OCaml polymorphism → Rust dyn Trait vs generics *)

(* Approach 1: Polymorphic variants / object types *)
class type shape = object
  method area : float
  method name : string
end

class circle r = object
  method area = Float.pi *. r *. r
  method name = "Circle"
end

class rectangle w h = object
  method area = w *. h
  method name = "Rectangle"
end

class triangle b h = object
  method area = 0.5 *. b *. h
  method name = "Triangle"
end

(* Approach 2: Using algebraic types with pattern matching *)
type shape_adt =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float

let area_adt = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
  | Triangle (b, h) -> 0.5 *. b *. h

let name_adt = function
  | Circle _ -> "Circle"
  | Rectangle _ -> "Rectangle"
  | Triangle _ -> "Triangle"

(* Approach 3: First-class modules *)
module type SHAPE = sig
  type t
  val create : float list -> t
  val area : t -> float
  val name : t -> string
end

module CircleMod : SHAPE = struct
  type t = float
  let create = function [r] -> r | _ -> failwith "need radius"
  let area r = Float.pi *. r *. r
  let name _ = "Circle"
end

(* Function that works with any shape (object approach) *)
let total_area (shapes : shape list) =
  List.fold_left (fun acc s -> acc +. s#area) 0.0 shapes

let describe (s : shape) =
  Printf.sprintf "%s: area=%.2f" s#name s#area

(* Tests *)
let () =
  (* Object approach *)
  let c = new circle 5.0 in
  let r = new rectangle 3.0 4.0 in
  let t = new triangle 6.0 3.0 in
  let shapes = [(c :> shape); (r :> shape); (t :> shape)] in
  let total = total_area shapes in
  assert (total > 99.0 && total < 101.0);

  (* ADT approach *)
  let c2 = Circle 5.0 in
  let r2 = Rectangle (3.0, 4.0) in
  assert (abs_float (area_adt c2 -. Float.pi *. 25.0) < 0.001);
  assert (abs_float (area_adt r2 -. 12.0) < 0.001);
  assert (name_adt c2 = "Circle");

  (* Dynamic dispatch via list *)
  let descriptions = List.map describe shapes in
  assert (List.length descriptions = 3);

  Printf.printf "✓ All tests passed\n"
