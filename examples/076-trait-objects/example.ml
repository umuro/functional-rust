(* 076: Trait Objects / Dynamic Dispatch
   OCaml uses first-class modules or records of functions for dynamic dispatch *)

(* --- Approach 1: Variant-based polymorphism (most common OCaml idiom) --- *)

type shape =
  | Circle    of float
  | Rectangle of float * float

let area = function
  | Circle r        -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h

let shape_name = function
  | Circle _    -> "circle"
  | Rectangle _ -> "rectangle"

(* --- Approach 2: Record-of-functions ("vtable" / trait object equivalent) ---
   This is the OCaml analogue to Rust's dyn Trait *)

type shape_obj = {
  area : unit -> float;
  name : unit -> string;
}

let make_circle r =
  { area = (fun () -> Float.pi *. r *. r)
  ; name = (fun () -> "circle") }

let make_rectangle w h =
  { area = (fun () -> w *. h)
  ; name = (fun () -> "rectangle") }

let describe s =
  Printf.sprintf "%s with area %.2f" (s.name ()) (s.area ())

let total_area shapes =
  List.fold_left (fun acc s -> acc +. s.area ()) 0.0 shapes

(* --- Approach 3: First-class modules (type-safe open polymorphism) --- *)

module type SHAPE = sig
  type t
  val area : t -> float
  val name : t -> string
end

let () =
  (* variant dispatch *)
  Printf.printf "circle area r=5: %.2f\n" (area (Circle 5.0));
  Printf.printf "rectangle 3x4: %.2f\n" (area (Rectangle (3.0, 4.0)));

  (* record-of-functions dispatch *)
  let shapes = [make_circle 5.0; make_rectangle 3.0 4.0] in
  List.iter (fun s -> Printf.printf "%s\n" (describe s)) shapes;
  Printf.printf "total area: %.2f\n" (total_area shapes)
