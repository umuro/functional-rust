(* 747: Test Fixtures — OCaml *)

(* Setup function: creates a temporary "database" *)
let setup_db () =
  let db = Hashtbl.create 16 in
  Hashtbl.add db "user:1" "Alice";
  Hashtbl.add db "user:2" "Bob";
  db

(* Teardown: in OCaml we just let GC collect it, but we can be explicit *)
let teardown_db db = Hashtbl.reset db

(* Test fixture wrapper — run test then teardown *)
let with_db f =
  let db = setup_db () in
  let result = (try Ok (f db) with e -> Error e) in
  teardown_db db;
  match result with
  | Ok () -> ()
  | Error e -> raise e

(* Tests using the fixture *)
let test_lookup_existing () =
  with_db (fun db ->
    match Hashtbl.find_opt db "user:1" with
    | Some "Alice" -> ()
    | _ -> failwith "expected Alice"
  )

let test_lookup_missing () =
  with_db (fun db ->
    assert (Hashtbl.find_opt db "user:99" = None)
  )

let test_insert_and_retrieve () =
  with_db (fun db ->
    Hashtbl.add db "user:3" "Carol";
    assert (Hashtbl.find_opt db "user:3" = Some "Carol")
  )

let () =
  test_lookup_existing ();
  test_lookup_missing ();
  test_insert_and_retrieve ();
  Printf.printf "Fixture tests passed!\n"
