let binary_to_decimal s =
  String.fold_left (fun acc c ->
    match c with
    | '0' -> acc * 2
    | '1' -> acc * 2 + 1
    | _ -> failwith "invalid binary digit"
  ) 0 s

let decimal_to_binary n =
  if n = 0 then "0"
  else
    let rec go n acc =
      if n = 0 then acc
      else go (n / 2) (string_of_int (n mod 2) ^ acc)
    in go n ""

let () =
  assert (binary_to_decimal "1010" = 10);
  assert (binary_to_decimal "11111" = 31);
  assert (decimal_to_binary 10 = "1010");
  assert (decimal_to_binary 0 = "0");
  List.iter (fun s ->
    let d = binary_to_decimal s in
    assert (decimal_to_binary d = s)
  ) ["1010"; "11111"; "101010"];
  print_endline "ok"
