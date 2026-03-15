(* 960: Key-Value Store *)
(* Simple in-memory KV store using Hashtbl *)

(* Approach 1: Hashtbl-based store with functional interface *)

type 'a store = (string, 'a) Hashtbl.t

let create () : 'a store = Hashtbl.create 16

let set store key value =
  Hashtbl.replace store key value

let get store key =
  Hashtbl.find_opt store key

let delete store key =
  Hashtbl.remove store key

let keys store =
  Hashtbl.fold (fun k _ acc -> k :: acc) store []

let values store =
  Hashtbl.fold (fun _ v acc -> v :: acc) store []

let size store = Hashtbl.length store

let contains store key = Hashtbl.mem store key

(* Approach 2: Functional (immutable) KV using association list *)

type 'a fstore = (string * 'a) list

let fset (store : 'a fstore) key value : 'a fstore =
  (key, value) :: List.filter (fun (k, _) -> k <> key) store

let fget (store : 'a fstore) key =
  List.assoc_opt key store

let fdelete (store : 'a fstore) key : 'a fstore =
  List.filter (fun (k, _) -> k <> key) store

let fkeys (store : 'a fstore) =
  List.map fst store

let () =
  (* Mutable store *)
  let s = create () in
  assert (size s = 0);

  set s "name" "Alice";
  set s "age" "30";
  set s "city" "Amsterdam";

  assert (get s "name" = Some "Alice");
  assert (get s "age" = Some "30");
  assert (get s "missing" = None);
  assert (contains s "city");
  assert (not (contains s "missing"));
  assert (size s = 3);

  (* Update existing key *)
  set s "name" "Bob";
  assert (get s "name" = Some "Bob");
  assert (size s = 3);

  (* Delete *)
  delete s "age";
  assert (get s "age" = None);
  assert (size s = 2);

  let ks = List.sort compare (keys s) in
  assert (ks = ["city"; "name"]);

  (* Functional store *)
  let fs = [] in
  let fs = fset fs "x" 1 in
  let fs = fset fs "y" 2 in
  let fs = fset fs "x" 10 in (* update *)
  assert (fget fs "x" = Some 10);
  assert (fget fs "y" = Some 2);
  assert (fget fs "z" = None);

  let fs = fdelete fs "y" in
  assert (fget fs "y" = None);
  assert (fkeys fs = ["x"]);

  Printf.printf "✓ All tests passed\n"
