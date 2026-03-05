(* Monad Pattern — Option and Result *)
(* Monadic bind for chaining fallible computations *)

(* Option monad *)
let ( >>= ) = Option.bind
let return x = Some x

let lookup_user id =
  if id = 1 then Some "Alice" else None

let lookup_email name =
  if name = "Alice" then Some "alice@example.com" else None

let get_email id =
  lookup_user id >>= lookup_email

let () = match get_email 1 with
  | Some e -> Printf.printf "Email: %s\n" e
  | None -> print_endline "Not found"

(* Result monad *)
let ( let* ) = Result.bind
let validate_age age =
  let* a = if age > 0 then Ok age else Error "non-positive" in
  let* _ = if a < 150 then Ok () else Error "too old" in
  Ok a

let () = match validate_age 25 with
  | Ok a -> Printf.printf "Valid age: %d\n" a
  | Error e -> Printf.printf "Error: %s\n" e
