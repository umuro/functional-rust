(* 1029: HashMap Entry API
   Rust's Entry API avoids double lookups for insert-or-update patterns.
   OCaml's Hashtbl provides find_opt + add/replace, or we use a helper
   that mirrors or_insert / and_modify semantics. *)

(* or_insert: insert a default if key absent, return current value *)
let or_insert tbl key default =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None   ->
    Hashtbl.add tbl key default;
    default

(* or_insert_with: compute default lazily *)
let or_insert_with tbl key f =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None   ->
    let v = f () in
    Hashtbl.add tbl key v;
    v

(* and_modify + or_insert: modify if present, insert default otherwise *)
let and_modify_or_insert tbl key modify_fn default =
  match Hashtbl.find_opt tbl key with
  | Some v ->
    let nv = modify_fn v in
    Hashtbl.replace tbl key nv;
    nv
  | None   ->
    Hashtbl.add tbl key default;
    default

let () =
  (* or_insert demo *)
  let m : (string, int) Hashtbl.t = Hashtbl.create 4 in
  Hashtbl.add m "a" 1;
  ignore (or_insert m "b" 42);
  ignore (or_insert m "a" 99);  (* "a" already exists — unchanged *)
  assert (Hashtbl.find m "a" = 1);
  assert (Hashtbl.find m "b" = 42);

  (* or_insert_with demo *)
  let m2 : (string, int) Hashtbl.t = Hashtbl.create 4 in
  List.iter (fun key ->
    ignore (or_insert_with m2 key (fun () ->
      match key with "x" -> 100 | "y" -> 200 | _ -> 0
    ))
  ) ["x"; "y"];
  assert (Hashtbl.find m2 "x" = 100);
  assert (Hashtbl.find m2 "y" = 200);

  (* and_modify demo *)
  let m3 : (string, int) Hashtbl.t = Hashtbl.create 4 in
  ignore (and_modify_or_insert m3 "count" (fun c -> c + 1) 1);
  assert (Hashtbl.find m3 "count" = 1);
  ignore (and_modify_or_insert m3 "count" (fun c -> c + 1) 1);
  assert (Hashtbl.find m3 "count" = 2);
  ignore (and_modify_or_insert m3 "count" (fun c -> c + 1) 1);
  assert (Hashtbl.find m3 "count" = 3);

  (* entry_ref: build char->position-list index *)
  let m4 : (char, int list) Hashtbl.t = Hashtbl.create 8 in
  String.iteri (fun i ch ->
    let lst = match Hashtbl.find_opt m4 ch with Some l -> l | None -> [] in
    Hashtbl.replace m4 ch (lst @ [i])
  ) "hello";
  assert (Hashtbl.find m4 'l' = [2; 3]);
  assert (Hashtbl.find m4 'h' = [0]);

  (* or_default equivalent *)
  let m5 : (string, int list) Hashtbl.t = Hashtbl.create 4 in
  let lst = or_insert_with m5 "nums" (fun () -> []) in
  let lst' = lst @ [42] in
  Hashtbl.replace m5 "nums" lst';
  assert (Hashtbl.find m5 "nums" = [42]);

  Printf.printf "Entry API tests passed\n"
