(* Streaming parser in OCaml using Seq (lazy sequences) *)

type record = { id: int; name: string; value: float }

let parse_record line =
  match String.split_on_char ',' line with
  | [id_s; name; val_s] ->
    (try Some { id = int_of_string (String.trim id_s);
                name = String.trim name;
                value = float_of_string (String.trim val_s) }
     with Failure _ -> None)
  | _ -> None

(* Streaming sequence from a string (simulate file) *)
let lines_of_string s =
  String.split_on_char '\n' s |> List.to_seq

let parse_stream input =
  lines_of_string input
  |> Seq.filter (fun l -> String.trim l <> "")
  |> Seq.filter_map parse_record

let () =
  let data = {|1, Alice, 95.5
2, Bob, 87.0
3, Carol, 100.0
bad,line
4, Dave, 72.3|} in
  Printf.printf "Streaming parse:\n";
  let count = ref 0 in
  Seq.iter (fun r ->
    incr count;
    Printf.printf "  Record %d: id=%d name=%s value=%.1f\n"
      !count r.id r.name r.value
  ) (parse_stream data);
  Printf.printf "Total valid records: %d\n" !count
