(* 491. Path handling – OCaml *)
let () =
  let path = "/home/user/documents/file.txt" in
  let parts = String.split_on_char '/' path |> List.filter ((<>) "") in
  Printf.printf "parts: %s\n" (String.concat " | " parts);
  let dir = String.concat "/" ("" :: List.rev (List.tl (List.rev parts))) in
  Printf.printf "dir: %s\n" dir;
  let base = List.nth parts (List.length parts - 1) in
  Printf.printf "file: %s\n" base;
  let ext = match String.rindex_opt base '.' with
    | Some i -> Some (String.sub base (i+1) (String.length base - i - 1))
    | None -> None
  in
  Printf.printf "ext: %s\n" (Option.value ~default:"none" ext)
