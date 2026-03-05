(* Applicative Functor Pattern *)
(* Applicative style for combining optional computations *)

(* Option as applicative *)
let ( <$> ) f x = Option.map f x
let ( <*> ) f x = match f with
  | None -> None
  | Some g -> Option.map g x

let safe_div x y = if y = 0 then None else Some (x / y)

let result =
  (fun a b c -> a + b + c)
  <$> Some 10
  <*> Some 20
  <*> Some 30

let () = match result with
  | Some n -> Printf.printf "Sum: %d\n" n
  | None -> print_endline "Failed"

(* Validate multiple fields *)
let parse name age =
  (fun n a -> (n, a))
  <$> (if name <> "" then Some name else None)
  <*> (if age > 0 && age < 150 then Some age else None)

let () = match parse "Alice" 30 with
  | Some (n, a) -> Printf.printf "%s is %d\n" n a
  | None -> print_endline "Invalid"
