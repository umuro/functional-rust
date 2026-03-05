(* 756: Tempfile Testing — OCaml *)

let unique_id =
  let counter = ref 0 in
  fun () -> incr counter; !counter

let create_temp_dir () =
  let tmp = Filename.get_temp_dir_name () in
  let name = Printf.sprintf "ocaml_test_%d_%d"
    (Unix.getpid ()) (unique_id ()) in
  let path = Filename.concat tmp name in
  Unix.mkdir path 0o700;
  path

let remove_dir_all path =
  (* Recursively remove directory *)
  let rec rm p =
    let entries = Sys.readdir p in
    Array.iter (fun entry ->
      let full = Filename.concat p entry in
      if Sys.is_directory full then rm full
      else Sys.remove full
    ) entries;
    Unix.rmdir p
  in
  if Sys.file_exists path then rm path

let with_temp_dir f =
  let dir = create_temp_dir () in
  let result = (try Ok (f dir) with e -> Error e) in
  remove_dir_all dir;
  match result with
  | Ok v  -> v
  | Error e -> raise e

let write_file path content =
  let oc = open_out path in
  output_string oc content;
  close_out oc

let read_file path =
  let ic = open_in path in
  let n = in_channel_length ic in
  let s = Bytes.create n in
  really_input ic s 0 n;
  close_in ic;
  Bytes.to_string s

let () =
  (* Test: write and read a file in temp dir *)
  with_temp_dir (fun dir ->
    let file = Filename.concat dir "test.txt" in
    write_file file "Hello, tempfile!";
    let content = read_file file in
    assert (content = "Hello, tempfile!");
    Printf.printf "Temp dir test passed: %s\n" dir
  );

  (* Dir should be cleaned up *)
  Printf.printf "Cleanup: temp dirs removed\n"
