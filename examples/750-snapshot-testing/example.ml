(* 750: Snapshot Testing — OCaml manual expect-file pattern *)

let read_file path =
  try
    let ic = open_in path in
    let n = in_channel_length ic in
    let s = Bytes.create n in
    really_input ic s 0 n;
    close_in ic;
    Some (Bytes.to_string s)
  with Sys_error _ -> None

let write_file path content =
  let oc = open_out path in
  output_string oc content;
  close_out oc

(* The function we're snapshot-testing *)
let render_report data =
  let lines = List.mapi (fun i (k, v) ->
    Printf.sprintf "%3d. %-20s %d" (i+1) k v
  ) data in
  "=== Sales Report ===\n"
  ^ String.concat "\n" lines
  ^ "\n==================\n"
  ^ Printf.sprintf "Total: %d items\n" (List.length data)

(* Snapshot assertion *)
let assert_snapshot name actual =
  let path = Printf.sprintf "tests/snapshots/%s.expected" name in
  match read_file path with
  | None ->
    (* First run: create snapshot *)
    (try Unix.mkdir "tests" 0o755 with Unix.Unix_error _ -> ());
    (try Unix.mkdir "tests/snapshots" 0o755 with Unix.Unix_error _ -> ());
    write_file path actual;
    Printf.printf "[snapshot:%s] Created snapshot\n" name
  | Some expected ->
    if actual = expected
    then Printf.printf "[snapshot:%s] OK\n" name
    else begin
      Printf.printf "[snapshot:%s] MISMATCH!\n" name;
      Printf.printf "Expected:\n%s\nActual:\n%s\n" expected actual;
      failwith "snapshot mismatch"
    end

let () =
  let data = [("Apples", 42); ("Bananas", 17); ("Cherries", 99)] in
  let report = render_report data in
  assert_snapshot "sales_report" report;
  Printf.printf "Snapshot tests done!\n"
