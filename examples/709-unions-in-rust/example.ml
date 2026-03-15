(* OCaml: algebraic variants ARE safe tagged unions. The compiler
   tracks the discriminant and guarantees exhaustive matching. *)

type value =
  | Int   of int
  | Float of float
  | Bool  of bool

let describe (v : value) : string =
  match v with
  | Int   n -> Printf.sprintf "Int(%d)"   n
  | Float f -> Printf.sprintf "Float(%g)" f
  | Bool  b -> Printf.sprintf "Bool(%b)"  b

let size_of_value (v : value) : int =
  match v with
  | Int   _ -> 8
  | Float _ -> 8
  | Bool  _ -> 1

let () =
  let vals = [Int 42; Float 3.14; Bool true; Int (-7)] in
  List.iter (fun v ->
    Printf.printf "%s (size=%d)\n" (describe v) (size_of_value v)
  ) vals
