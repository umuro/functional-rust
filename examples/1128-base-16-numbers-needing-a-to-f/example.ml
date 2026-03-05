(* Base 16 numbers needing a to f *)
(* Rosetta Code Base 16 numbers needing a to f implementation in OCaml *)

let rec has_xdigit n =
  n land 15 > 9 || n > 15 && has_xdigit (n lsr 4)

let () =
  Seq.(ints 1 |> take 500 |> filter has_xdigit |> map string_of_int)
  |> List.of_seq |> String.concat " " |> print_endline
