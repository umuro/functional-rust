(* 257: Pairing elements with zip
   OCaml: List.combine stops at the shorter list (like Rust's zip).
   Seq.zip is the lazy equivalent. *)

let () =
  (* Basic zip — pairs matching positions *)
  let a = [1; 2; 3] and b = ["x"; "y"; "z"] in
  let pairs = List.combine a b in
  Printf.printf "zip [1;2;3] [x;y;z] = [%s]\n"
    (pairs |> List.map (fun (n, s) -> Printf.sprintf "(%d,%s)" n s)
           |> String.concat ";");

  (* Zip truncates at the shorter list *)
  let long  = [1;2;3;4;5] and short = [10;20] in
  let trunc = List.combine (List.filteri (fun i _ -> i < List.length short) long) short in
  Printf.printf "zip (truncated) = [%s]\n"
    (trunc |> List.map (fun (a,b) -> Printf.sprintf "(%d,%d)" a b)
           |> String.concat ";");

  (* Seq.zip — lazy, stops at shorter *)
  let seq_a = List.to_seq [1;2;3;4;5] and seq_b = List.to_seq [10;20] in
  let zipped = Seq.zip seq_a seq_b |> List.of_seq in
  Printf.printf "seq zip count = %d\n" (List.length zipped);

  (* Build a string->int map from two lists *)
  let keys = ["a"; "b"; "c"] and vals = [1; 2; 3] in
  let tbl = Hashtbl.create 8 in
  List.iter2 (fun k v -> Hashtbl.add tbl k v) keys vals;
  Printf.printf "map[\"a\"] = %d, map[\"b\"] = %d\n"
    (Hashtbl.find tbl "a") (Hashtbl.find tbl "b");

  (* Unzip back to two lists *)
  let (xs, ys) = List.split pairs in
  Printf.printf "unzipped: [%s] [%s]\n"
    (xs |> List.map string_of_int |> String.concat ";")
    (String.concat ";" ys)
