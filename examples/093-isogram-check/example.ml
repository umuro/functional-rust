let is_isogram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z')
    |> List.of_seq in
  let unique = List.sort_uniq Char.compare chars in
  List.length chars = List.length unique

let () =
  List.iter (fun s ->
    Printf.printf "%s: %b\n" s (is_isogram s)
  ) ["lumberjacks"; "background"; "eleven"; "subdermatoglyphic"]
