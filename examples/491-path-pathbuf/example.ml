(* 491: Path / PathBuf — file path manipulation in OCaml *)
(* OCaml uses the Filename module from the standard library.
   Filename is platform-aware (uses '/' on Unix, '\' on Windows). *)

(* Join path components — analogous to PathBuf::join *)
let path_join parts =
  List.fold_left Filename.concat (List.hd parts) (List.tl parts)

(* Extract the file extension — analogous to Path::extension *)
let extension path =
  let base = Filename.basename path in
  (* Find the last '.' after position 0 (ignoring dotfiles) *)
  match String.rindex_opt base '.' with
  | None   -> None
  | Some i ->
    if i = 0 then None   (* dotfile like ".hidden" — no extension *)
    else Some (String.sub base (i + 1) (String.length base - i - 1))

(* Extract the file stem (basename without extension) — analogous to Path::file_stem *)
let file_stem path =
  let base = Filename.basename path in
  match String.rindex_opt base '.' with
  | None   -> base
  | Some 0 -> base   (* dotfile — whole name is the stem *)
  | Some i -> String.sub base 0 i

(* Get the parent directory — analogous to Path::parent *)
let parent path =
  let p = Filename.dirname path in
  if p = path then None  (* at root or bare filename with no dir *)
  else Some p

let () =
  (* join *)
  let joined = path_join ["/a"; "b"; "c"] in
  assert (joined = "/a/b/c");
  Printf.printf "join: %s\n" joined;

  (* extension *)
  assert (extension "f.txt" = Some "txt");
  assert (extension "f.tar.gz" = Some "gz");
  assert (extension "noext" = None);
  print_endline "extension: ok";

  (* file_stem *)
  assert (file_stem "f.txt" = "f");
  assert (file_stem "noext" = "noext");
  print_endline "file_stem: ok";

  (* parent *)
  assert (parent "/a/b/c" = Some "/a/b");
  assert (parent "/a/b"   = Some "/a");
  print_endline "parent: ok";

  print_endline "All assertions passed."
