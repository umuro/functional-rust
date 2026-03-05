(* Polonius-style patterns in OCaml — all trivially safe with GC *)
let get_or_insert tbl key default_fn =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None ->
    let v = default_fn () in
    Hashtbl.add tbl key v;
    v

let () =
  let map = Hashtbl.create 4 in
  let v1 = get_or_insert map "key" (fun () -> "new_value") in
  let v2 = get_or_insert map "key" (fun () -> "another") in
  Printf.printf "v1=%s, v2=%s\n" v1 v2;

  (* Pattern that confuses NLL: get-or-insert *)
  let find_or_create arr key value =
    match Array.exists (fun (k, _) -> k = key) arr with
    | true -> arr
    | false -> Array.append arr [| (key, value) |]
  in
  let db = find_or_create [| ("a", 1) |] "b" 2 in
  Array.iter (fun (k, v) -> Printf.printf "%s=%d " k v) db;
  print_newline ()
