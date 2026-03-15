(* 076: Trait Objects — OCaml objects for dynamic dispatch *)

(* Approach 1: OCaml object types *)
class virtual shape = object
  method virtual area : float
  method virtual name : string
end

class circle r = object
  inherit shape
  method area = Float.pi *. r *. r
  method name = "circle"
end

class rectangle w h = object
  inherit shape
  method area = w *. h
  method name = "rectangle"
end

(* Approach 2: Using objects polymorphically *)
let describe (s : shape) =
  Printf.sprintf "%s with area %.2f" s#name s#area

let total_area (shapes : shape list) =
  List.fold_left (fun acc s -> acc +. s#area) 0.0 shapes

(* Approach 3: First-class modules as alternative *)
module type SHAPE = sig
  val area : unit -> float
  val name : unit -> string
end

let make_circle r : (module SHAPE) =
  (module struct
    let area () = Float.pi *. r *. r
    let name () = "circle"
  end)

let describe_mod (module S : SHAPE) =
  Printf.sprintf "%s with area %.2f" (S.name ()) (S.area ())

(* Tests *)
let () =
  let c = new circle 5.0 in
  let r = new rectangle 3.0 4.0 in
  assert (abs_float (c#area -. 78.54) < 0.01);
  assert (r#area = 12.0);
  assert (c#name = "circle");
  let shapes = [c :> shape; r :> shape] in
  assert (abs_float (total_area shapes -. 90.54) < 0.01);
  let mc = make_circle 5.0 in
  let desc = describe_mod mc in
  assert (String.sub desc 0 6 = "circle");
  Printf.printf "✓ All tests passed\n"
