(* Polymorphic Variants *)
(* Open variant types with backtick syntax *)

let describe_color = function
  | `Red -> "red"
  | `Green -> "green"
  | `Blue -> "blue"
  | `Custom (r, g, b) -> Printf.sprintf "rgb(%d,%d,%d)" r g b

let is_primary = function
  | `Red | `Green | `Blue -> true
  | `Custom _ -> false

let colors = [`Red; `Blue; `Custom (128, 0, 255)]
let () = List.iter (fun c ->
  Printf.printf "%s (primary: %b)\n" (describe_color c) (is_primary c)
) colors
