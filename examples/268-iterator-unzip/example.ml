(* 268: unzip — split a list of pairs into two separate lists.
   OCaml: List.split does exactly this. *)

let () =
  (* Basic unzip *)
  let pairs = [(1, 'a'); (2, 'b'); (3, 'c')] in
  let (nums, chars) = List.split pairs in
  Printf.printf "nums  = [%s]\n"
    (nums |> List.map string_of_int |> String.concat ";");
  Printf.printf "chars = [%s]\n"
    (chars |> List.map (String.make 1) |> String.concat ";");

  (* Round-trip: zip then unzip *)
  let a = [1;2;3] and b = [4;5;6] in
  let zipped = List.combine a b in
  let (a2, b2) = List.split zipped in
  Printf.printf "roundtrip a: %b\n" (a = a2);
  Printf.printf "roundtrip b: %b\n" (b = b2);

  (* Empty unzip *)
  let (ea, eb) = List.split ([] : (int * int) list) in
  Printf.printf "empty unzip: |a|=%d |b|=%d\n"
    (List.length ea) (List.length eb);

  (* Practical: separate key-value associations *)
  let assoc = [("name", "Alice"); ("city", "Paris"); ("age", "30")] in
  let (keys, values) = List.split assoc in
  Printf.printf "keys   = [%s]\n" (String.concat ";" keys);
  Printf.printf "values = [%s]\n" (String.concat ";" values);

  (* Unzip a sequence of integer pairs *)
  let int_pairs = List.mapi (fun i _ -> (i, i * i)) [0;0;0;0;0] in
  let (indices, squares) = List.split int_pairs in
  Printf.printf "indices = [%s]\n"
    (indices |> List.map string_of_int |> String.concat ";");
  Printf.printf "squares = [%s]\n"
    (squares |> List.map string_of_int |> String.concat ";")
