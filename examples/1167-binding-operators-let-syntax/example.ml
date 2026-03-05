(* Binding Operators (let* syntax) *)
(* Modern OCaml binding operators for monadic code *)

(* Define binding operators for Option *)
let ( let* ) = Option.bind
let ( let+ ) x f = Option.map f x
let ( and+ ) a b = match (a, b) with
  | (Some x, Some y) -> Some (x, y)
  | _ -> None

let parse_pair s1 s2 =
  let+ (a, b) = int_of_string_opt s1 and+ int_of_string_opt s2 in
  a + b

(* Result binding operators *)
module ResultSyntax = struct
  let ( let* ) = Result.bind
  let ( let+ ) x f = Result.map f x
end

let () =
  let open ResultSyntax in
  let result =
    let* x = Ok 10 in
    let* y = Ok 20 in
    let+ z = Ok 30 in
    x + y + z
  in
  match result with Ok n -> Printf.printf "%d\n" n | Error _ -> ()
