(* Option.map and Option.bind — Safe Value Transformation *)
(* Chain operations on optional values *)

let parse_int s = match int_of_string_opt s with Some n -> Some n | None -> None
let safe_div x y = if y = 0 then None else Some (x / y)

let result =
  parse_int "42"
  |> Option.map (fun x -> x * 2)
  |> Option.bind (fun x -> safe_div x 7)

let () = match result with
  | Some v -> Printf.printf "Result: %d\n" v
  | None -> Printf.printf "No result\n"
