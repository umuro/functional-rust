(* 097: Flatten and Flat Map
   OCaml: List.concat, List.concat_map, Seq.flat_map *)

(* --- Approach 1: flatten — concatenate a list of lists --- *)

let flatten xss = List.concat xss

(* --- Approach 2: flat_map — map then flatten in one pass --- *)

let flat_map f xs = List.concat_map f xs

(* --- Approach 3: flatten on Options (filter_map) --- *)

let flatten_options opts = List.filter_map Fun.id opts

(* --- Approach 4: Seq.flat_map for lazy flattening --- *)

let seq_flat_map f seq =
  Seq.flat_map f seq

let () =
  (* flatten *)
  let nested = [[1;2]; [3;4]; [5]] in
  Printf.printf "flatten [[1;2];[3;4];[5]] = [%s]\n"
    (String.concat "; " (List.map string_of_int (flatten nested)));

  (* flat_map *)
  let expanded = flat_map (fun x -> [x; x * 10]) [1;2;3] in
  Printf.printf "flat_map (x -> [x; x*10]) [1;2;3] = [%s]\n"
    (String.concat "; " (List.map string_of_int expanded));

  (* flatten with empty lists *)
  let with_empty = [[]; [1]; []; [2;3]] in
  Printf.printf "flatten with empty = [%s]\n"
    (String.concat "; " (List.map string_of_int (flatten with_empty)));

  (* flatten options *)
  let opts = [Some 1; None; Some 3] in
  Printf.printf "flatten_options = [%s]\n"
    (String.concat "; " (List.map string_of_int (flatten_options opts)));

  (* lazy seq flat_map *)
  let result =
    List.to_seq [1;2;3]
    |> seq_flat_map (fun x -> List.to_seq [x; x * 10])
    |> List.of_seq
  in
  Printf.printf "seq flat_map = [%s]\n"
    (String.concat "; " (List.map string_of_int result))
