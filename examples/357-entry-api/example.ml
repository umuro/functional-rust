(* 357: Entry API
   Rust's HashMap entry API allows efficient insert-or-update.
   In OCaml we replicate this with Hashtbl helpers that avoid
   double lookups. *)

(* Count characters — like entry().and_modify().or_insert(1) *)
let count_chars s =
  let tbl = Hashtbl.create 16 in
  String.iter (fun c ->
    let n = try Hashtbl.find tbl c with Not_found -> 0 in
    Hashtbl.replace tbl c (n + 1)
  ) s;
  tbl

(* Get existing value or compute and insert a default *)
let get_or_compute tbl key compute =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None   ->
    let v = compute () in
    Hashtbl.add tbl key v;
    v

(* Update existing value if present; otherwise insert default *)
let update_with_default tbl key default update =
  match Hashtbl.find_opt tbl key with
  | Some v -> Hashtbl.replace tbl key (update v)
  | None   -> Hashtbl.add tbl key default

let () =
  (* Character counting *)
  let counts = count_chars "hello" in
  assert (Hashtbl.find counts 'l' = 2);
  assert (Hashtbl.find counts 'o' = 1);
  Printf.printf "count 'l'=%d 'o'=%d\n%!"
    (Hashtbl.find counts 'l') (Hashtbl.find counts 'o');

  (* Lazy compute: computed once, not again *)
  let tbl = Hashtbl.create 4 in
  let v  = get_or_compute tbl "key" (fun () -> 42) in
  assert (v = 42);
  let v2 = get_or_compute tbl "key" (fun () -> 99) in
  assert (v2 = 42);  (* not recomputed *)
  Printf.printf "get_or_compute: %d (not recomputed: %d)\n%!" v v2;

  (* Update existing *)
  let tbl2 = Hashtbl.create 4 in
  Hashtbl.add tbl2 "k" 10;
  update_with_default tbl2 "k" 0 (fun v -> v * 2);
  assert (Hashtbl.find tbl2 "k" = 20);
  Printf.printf "update existing: %d\n%!" (Hashtbl.find tbl2 "k");

  (* Insert default when absent *)
  update_with_default tbl2 "new" 5 (fun v -> v * 2);
  assert (Hashtbl.find tbl2 "new" = 5);
  Printf.printf "insert default: %d\n%!" (Hashtbl.find tbl2 "new")
