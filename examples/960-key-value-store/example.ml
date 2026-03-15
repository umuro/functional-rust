(* 960: Key-Value Store

   OCaml: two approaches
   1. Mutable — Hashtbl (like Rust's HashMap)
   2. Functional (immutable) — association list or Map.Make *)

(* ── Approach 1: Mutable KV store using Hashtbl ─────────────────────────── *)

module KvStore = struct
  type t = (string, string) Hashtbl.t

  let create () : t = Hashtbl.create 16

  let set (store : t) key value =
    Hashtbl.replace store key value

  let get (store : t) key =
    Hashtbl.find_opt store key

  let delete (store : t) key =
    let existed = Hashtbl.mem store key in
    Hashtbl.remove store key;
    existed

  let contains (store : t) key =
    Hashtbl.mem store key

  let keys (store : t) =
    let ks = Hashtbl.fold (fun k _ acc -> k :: acc) store [] in
    List.sort compare ks   (* sorted, like the Rust version *)

  let values (store : t) =
    Hashtbl.fold (fun _ v acc -> v :: acc) store []

  let size (store : t) = Hashtbl.length store

  let is_empty (store : t) = Hashtbl.length store = 0
end

(* ── Approach 2: Functional (immutable) store using Map.Make ─────────────── *)

module StringMap = Map.Make(String)

module FunctionalStore = struct
  type 'v t = 'v StringMap.t

  let empty : 'v t = StringMap.empty

  let set key value store = StringMap.add key value store

  let get key store = StringMap.find_opt key store

  let delete key store = StringMap.remove key store

  let contains key store = StringMap.mem key store

  let keys store = List.map fst (StringMap.bindings store)

  let size store = StringMap.cardinal store

  let is_empty store = StringMap.is_empty store

  (* to_list: sorted key-value pairs *)
  let to_list store = StringMap.bindings store
end

(* ── Approach 3: Association-list (purely functional, O(n) lookup) ─────── *)

module AlistStore = struct
  type 'v t = (string * 'v) list

  let empty : 'v t = []

  let set key value store =
    (* Remove old binding, prepend new *)
    (key, value) :: List.filter (fun (k, _) -> k <> key) store

  let get key store =
    List.assoc_opt key store

  let delete key store =
    List.filter (fun (k, _) -> k <> key) store

  let contains key store =
    List.mem_assoc key store

  let keys store = List.map fst store

  let size store = List.length store
end

let () =
  (* Mutable KvStore *)
  let s = KvStore.create () in
  assert (KvStore.size s = 0);
  assert (KvStore.is_empty s);

  KvStore.set s "name" "Alice";
  KvStore.set s "age" "30";
  KvStore.set s "city" "Amsterdam";

  assert (KvStore.get s "name" = Some "Alice");
  assert (KvStore.get s "age"  = Some "30");
  assert (KvStore.get s "missing" = None);
  assert (KvStore.contains s "city");
  assert (not (KvStore.contains s "missing"));
  assert (KvStore.size s = 3);

  (* update *)
  KvStore.set s "name" "Bob";
  assert (KvStore.get s "name" = Some "Bob");
  assert (KvStore.size s = 3);

  (* delete *)
  let removed = KvStore.delete s "age" in
  assert removed;
  assert (KvStore.get s "age" = None);
  assert (KvStore.size s = 2);
  assert (not (KvStore.delete s "missing"));

  (* keys sorted *)
  KvStore.set s "name" "Alice";
  KvStore.set s "age" "30";
  assert (KvStore.keys s = ["age"; "city"; "name"]);

  (* Functional store — immutable Map-based *)
  let open FunctionalStore in
  let fs = empty in
  let fs = set "x" 1 fs in
  let fs = set "y" 2 fs in
  let fs = set "x" 10 fs in  (* update — creates new store *)
  assert (get "x" fs = Some 10);
  assert (get "y" fs = Some 2);
  assert (get "z" fs = None);
  let fs2 = delete "y" fs in
  assert (get "y" fs2 = None);
  assert (get "y" fs  = Some 2);  (* original unchanged *)
  assert (keys fs = ["x"; "y"]);  (* sorted *)

  (* Association-list store *)
  let open AlistStore in
  let al = empty in
  let al = set "a" 1 al in
  let al = set "b" 2 al in
  let al = set "a" 99 al in  (* update *)
  assert (get "a" al = Some 99);
  assert (get "b" al = Some 2);
  assert (get "c" al = None);
  let al2 = delete "b" al in
  assert (not (contains "b" al2));
  assert (get "a" al = Some 99);  (* original unchanged *)

  print_endline "960-key-value-store: all tests passed"
