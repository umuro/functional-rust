(* OCaml: copy-on-write via Bytes *)

let maybe_uppercase s threshold =
  if String.length s > threshold then String.uppercase_ascii s
  else s  (* no copy needed *)

let process_strings strs =
  List.map (fun s ->
    let processed = maybe_uppercase s 5 in
    processed
  ) strs

let () =
  let strs = ["hello"; "hi"; "world"; "rust"; "programming"] in
  List.iter (fun s -> Printf.printf "%s\n" s) (process_strings strs)
