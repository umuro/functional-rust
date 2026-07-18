(* Option Basics *)
(* OCaml 99 Problems #41 *)

let safe_min = function
  | [] -> None
  | x :: xs -> Some (List.fold_left min x xs)

let describe = function
  | Some x -> Printf.sprintf "Some(%d)" x
  | None -> "None"

let value_or_default opt default = Option.value opt ~default

(* Tests *)
let () =
  assert (safe_min [3; 1; 4; 1; 5] = Some 1);
  assert (safe_min [] = None);
  assert (describe (Some 42) = "Some(42)");
  assert (describe None = "None");
  assert (value_or_default (Some 5) 0 = 5);
  assert (value_or_default None 0 = 0);
  print_endline "✓ OCaml tests passed"
