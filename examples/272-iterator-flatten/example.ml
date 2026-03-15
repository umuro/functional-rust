(* 272: flatten — remove one level of nesting.
   OCaml: List.flatten (= List.concat); Option nesting uses Option.join. *)

let () =
  (* Flatten nested lists — one level only *)
  let nested = [[1;2]; [3;4]] in
  let flat = List.flatten nested in
  Printf.printf "flatten [[1;2];[3;4]] = [%s]\n"
    (flat |> List.map string_of_int |> String.concat ";");

  (* Option.join: flatten one level of Option nesting *)
  Printf.printf "Option.join (Some (Some 42)) = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (Option.join (Some (Some 42))));
  Printf.printf "Option.join (Some None)       = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (Option.join (Some (None : int option))));
  Printf.printf "Option.join None              = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (Option.join (None : int option option)));

  (* Flatten a list of options — extract Somes *)
  let opts = [Some 1; None; Some 3] in
  let values = List.filter_map Fun.id opts in
  Printf.printf "filter_map id [Some 1; None; Some 3] = [%s]\n"
    (values |> List.map string_of_int |> String.concat ";");

  (* Lazy flatten using Seq *)
  let seq_nested = List.to_seq [[1;2;3]; [4;5]] in
  let seq_flat = seq_nested |> Seq.flat_map List.to_seq |> List.of_seq in
  Printf.printf "seq flatten = [%s]\n"
    (seq_flat |> List.map string_of_int |> String.concat ";");

  (* Three levels: flatten removes only one level at a time *)
  let deep = [[[1;2]; [3]]; [[4]]] in
  let one_level = List.flatten deep in  (* still nested *)
  let two_levels = List.flatten one_level in
  Printf.printf "deep after 1 flatten: %d sublists\n" (List.length one_level);
  Printf.printf "deep after 2 flatten: [%s]\n"
    (two_levels |> List.map string_of_int |> String.concat ";")
