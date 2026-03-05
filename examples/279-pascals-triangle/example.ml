let next_row row =
  List.map2 (+) (0 :: row) (row @ [0])

let pascal n =
  let rec go row i =
    if i > n then []
    else row :: go (next_row row) (i + 1)
  in go [1] 1

let () =
  pascal 8 |> List.iter (fun row ->
    List.iter (Printf.printf "%d ") row;
    print_newline ()
  );
  (* Assertions *)
  assert (next_row [1] = [1; 1]);
  assert (next_row [1; 1] = [1; 2; 1]);
  assert (List.length (pascal 8) = 8);
  let row7 = List.nth (pascal 8) 7 in
  assert (row7 = [1; 7; 21; 35; 35; 21; 7; 1]);
  print_endline "ok"
