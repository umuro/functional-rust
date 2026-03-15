(* Matrix Operations — Functional 2D *)

let transpose matrix =
  match matrix with
  | [] -> []
  | _ -> List.init (List.length (List.hd matrix)) (fun i ->
    List.map (fun row -> List.nth row i) matrix)

let dot a b = List.fold_left2 (fun acc x y -> acc + x * y) 0 a b

let multiply a b =
  let bt = transpose b in
  List.map (fun row -> List.map (dot row) bt) a

let print_matrix m =
  List.iter (fun row ->
    List.iter (Printf.printf "%3d ") row;
    print_newline ()
  ) m

let () =
  let a = [[1;2;3];[4;5;6]] in
  let b = [[7;8];[9;10];[11;12]] in
  print_matrix (multiply a b);
  assert (multiply a b = [[58;64];[139;154]]);
  Printf.printf "Matrix tests passed!\n"
