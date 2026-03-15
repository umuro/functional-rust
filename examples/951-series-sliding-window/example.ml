(* Idiomatic OCaml — List.init + String.sub *)
let series n s =
  if n > String.length s then []
  else
    List.init (String.length s - n + 1) (fun i ->
      String.sub s i n
    )

(* Recursive OCaml — explicit tail recursion with accumulator *)
let rec series_rec n s acc i =
  if i + n > String.length s then List.rev acc
  else series_rec n s (String.sub s i n :: acc) (i + 1)

let series_recursive n s =
  if n > String.length s then []
  else series_rec n s [] 0

let largest_product n s =
  if n = 0 then Ok 1
  else if n > String.length s then Error "span too large"
  else
    series n s
    |> List.map (fun sub ->
      String.fold_left (fun acc c ->
        acc * (Char.code c - Char.code '0')
      ) 1 sub
    )
    |> List.fold_left max 0
    |> fun m -> Ok m

let () =
  assert (series 3 "49142" = ["491"; "914"; "142"]);
  assert (series 6 "49142" = []);
  assert (series_recursive 3 "49142" = ["491"; "914"; "142"]);
  assert (largest_product 0 "12345" = Ok 1);
  assert (largest_product 2 "0123456789" = Ok 72);
  assert (largest_product 6 "49142" = Error "span too large");
  List.iter (Printf.printf "%s ") (series 3 "49142");
  print_newline ();
  (match largest_product 2 "0123456789" with
  | Ok n -> Printf.printf "Largest: %d\n" n
  | Error e -> Printf.printf "Error: %s\n" e);
  print_endline "ok"
