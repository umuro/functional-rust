(* Type dispatch via sum types in OCaml *)
type value = I of int | F of float | S of string | B of bool

let type_name = function
  | I _ -> "int" | F _ -> "float" | S _ -> "string" | B _ -> "bool"

let to_f64 = function
  | I n -> Some (float_of_int n)
  | F f -> Some f
  | S s -> (try Some (float_of_string s) with _ -> None)
  | _   -> None

let show = function
  | I n -> string_of_int n | F f -> string_of_float f
  | S s -> Printf.sprintf "%S" s | B b -> string_of_bool b

let () =
  let vs = [I 42; F 3.14; S "hello"; B true] in
  List.iter (fun v -> Printf.printf "%s : %s\n" (show v) (type_name v)) vs;
  List.iter (fun v -> match to_f64 v with Some f->Printf.printf "%.2f\n" f | None->()) vs
