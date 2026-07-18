(* Result Map *)
(* OCaml 99 Problems #46 *)

let transform r =
  r
  |> Result.map (fun x -> string_of_int (x * 2))
  |> Result.map_error (fun e -> "Error: " ^ e)

(* Tests *)
let () =
  assert (transform (Ok 21) = Ok "42");
  assert (transform (Error "bad input") = Error "Error: bad input");

  let r : (int, string) result = Error "x" in
  assert (Result.map (fun x -> x * 2) r = Error "x");

  let ok_r : (int, string) result = Ok 5 in
  assert (Result.map_error (fun e -> "Error: " ^ e) ok_r = Ok 5);

  print_endline "✓ OCaml tests passed"
