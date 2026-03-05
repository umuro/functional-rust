(* Hashtbl — Default Dict Pattern *)
(* Hash table with default value factory *)

let find_or_add tbl key default_fn =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None ->
    let v = default_fn () in
    Hashtbl.add tbl key v; v

(* Group items by category *)
let group_by key_fn items =
  let tbl = Hashtbl.create 16 in
  List.iter (fun item ->
    let key = key_fn item in
    let lst = find_or_add tbl key (fun () -> ref []) in
    lst := item :: !lst
  ) items;
  Hashtbl.fold (fun k v acc -> (k, List.rev !v) :: acc) tbl []

let data = ["apple"; "banana"; "avocado"; "blueberry"; "cherry"]
let groups = group_by (fun s -> s.[0]) data
let () = List.iter (fun (k, vs) ->
  Printf.printf "%c: %s\n" k (String.concat ", " vs)
) groups
