(* 285. Building custom iterator adapters - OCaml *)

(* Custom adapter: every_nth -- yield every nth element *)
let every_nth n seq =
  Seq.unfold (fun (i, rest) ->
    let rec skip_to k s =
      if k = 0 then
        match Seq.uncons s with
        | Some (v, rest') -> Some (v, (n-1, rest'))
        | None -> None
      else
        match Seq.uncons s with
        | Some (_, rest') -> skip_to (k-1) rest'
        | None -> None
    in
    skip_to i rest
  ) (0, seq)

(* Custom adapter: running_window -- sliding running tuple *)
let pairs seq =
  Seq.unfold (fun s ->
    match Seq.uncons s with
    | None -> None
    | Some (a, rest) ->
      match Seq.uncons rest with
      | None -> None
      | Some (b, _) -> Some ((a, b), Seq.drop 1 s)
  ) seq

let () =
  let nums = List.to_seq [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in
  let thirds = every_nth 3 nums in
  Printf.printf "Every 3rd: %s\n"
    (String.concat ", " (List.map string_of_int (List.of_seq thirds)));

  let data = List.to_seq [10; 20; 30; 40; 50] in
  let adjacent = pairs data in
  List.iter (fun (a, b) -> Printf.printf "(%d,%d) " a b) (List.of_seq adjacent);
  print_newline ()
