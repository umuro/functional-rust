(* 359: Multimap Pattern
   A map where each key can hold multiple values.
   OCaml: Hashtbl with list values, or Map functor with list values. *)

(* Mutable multimap using Hashtbl *)
type ('k, 'v) multimap = ('k, 'v list) Hashtbl.t

let create () : ('k, 'v) multimap = Hashtbl.create 16

let insert (mm : ('k, 'v) multimap) key value =
  let xs = try Hashtbl.find mm key with Not_found -> [] in
  Hashtbl.replace mm key (xs @ [value])

let get_all (mm : ('k, 'v) multimap) key =
  try Hashtbl.find mm key with Not_found -> []

let count (mm : ('k, 'v) multimap) key =
  List.length (get_all mm key)

(* Remove the last inserted value for a key *)
let remove_one (mm : ('k, 'v) multimap) key =
  match Hashtbl.find_opt mm key with
  | None | Some [] -> None
  | Some xs ->
    let n = List.length xs in
    let last = List.nth xs (n - 1) in
    let rest = List.filteri (fun i _ -> i < n - 1) xs in
    if rest = [] then Hashtbl.remove mm key
    else Hashtbl.replace mm key rest;
    Some last

(* Immutable multimap using Map + list — functional style *)
module StringMap = Map.Make(String)

let imm_insert map key value =
  let xs = try StringMap.find key map with Not_found -> [] in
  StringMap.add key (xs @ [value]) map

let imm_get_all map key =
  try StringMap.find key map with Not_found -> []

let () =
  (* Mutable multimap *)
  let mm = create () in
  insert mm "tags" "rust";
  insert mm "tags" "async";
  assert (count mm "tags" = 2);
  Printf.printf "tags count: %d\n%!" (count mm "tags");

  let vals = get_all mm "tags" in
  assert (vals = ["rust";"async"]);
  Printf.printf "tags values: %s\n%!"
    (vals |> String.concat ", ");

  insert mm 1 "a"; insert mm 1 "b";
  assert (remove_one mm 1 = Some "b");
  assert (count mm 1 = 1);
  Printf.printf "remove_one: remaining count=%d\n%!" (count mm 1);

  (* Immutable multimap *)
  let imap = StringMap.empty in
  let imap = imm_insert imap "lang" "ocaml" in
  let imap = imm_insert imap "lang" "rust" in
  assert (imm_get_all imap "lang" = ["ocaml";"rust"]);
  Printf.printf "immutable multimap lang: %s\n%!"
    (imm_get_all imap "lang" |> String.concat ", ")
