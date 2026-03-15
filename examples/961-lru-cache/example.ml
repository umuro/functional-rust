(* 961: LRU Cache
   OCaml: Hashtbl for O(1) lookup + doubly-linked list for O(1) recency tracking.
   The key insight: a doubly-linked list lets us move any node to the front in O(1),
   while Hashtbl maps keys directly to their list nodes. *)

(* Doubly-linked list node *)
type 'a node = {
  key : string;
  mutable value : 'a;
  mutable prev : 'a node option;
  mutable next : 'a node option;
}

type 'a lru_cache = {
  capacity : int;
  table : (string, 'a node) Hashtbl.t;
  (* sentinel head/tail — simplifies boundary cases *)
  head : 'a node;
  tail : 'a node;
}

let make_sentinel () = { key = ""; value = Obj.magic (); prev = None; next = None }

let create capacity =
  assert (capacity > 0);
  let head = make_sentinel () in
  let tail = make_sentinel () in
  head.next <- Some tail;
  tail.prev <- Some head;
  { capacity; table = Hashtbl.create capacity; head; tail }

(* Remove a node from the doubly-linked list *)
let remove_node node =
  (match node.prev with Some p -> p.next <- node.next | None -> ());
  (match node.next with Some n -> n.prev <- node.prev | None -> ());
  node.prev <- None;
  node.next <- None

(* Insert a node right after head (most-recently-used position) *)
let insert_after_head cache node =
  let second = cache.head.next in
  cache.head.next <- Some node;
  node.prev <- Some cache.head;
  node.next <- second;
  (match second with Some s -> s.prev <- Some node | None -> ())

let get cache key =
  match Hashtbl.find_opt cache.table key with
  | None -> None
  | Some node ->
    (* Move to front: most recently used *)
    remove_node node;
    insert_after_head cache node;
    Some node.value

let put cache key value =
  match Hashtbl.find_opt cache.table key with
  | Some node ->
    (* Update existing: move to front *)
    node.value <- value;
    remove_node node;
    insert_after_head cache node
  | None ->
    (* Evict LRU (node just before tail) if at capacity *)
    if Hashtbl.length cache.table >= cache.capacity then begin
      match cache.tail.prev with
      | Some lru when lru != cache.head ->
        remove_node lru;
        Hashtbl.remove cache.table lru.key
      | _ -> ()
    end;
    let node = { key; value; prev = None; next = None } in
    Hashtbl.add cache.table key node;
    insert_after_head cache node

let size cache = Hashtbl.length cache.table
let contains cache key = Hashtbl.mem cache.table key

let () =
  let cache : int lru_cache = create 3 in
  put cache "a" 1;
  put cache "b" 2;
  put cache "c" 3;
  Printf.printf "get a = %s\n" (match get cache "a" with Some v -> string_of_int v | None -> "None");
  (* Access "a" → now "b" is LRU. Insert "d" → evicts "b". *)
  put cache "d" 4;
  Printf.printf "contains b = %b\n" (contains cache "b");  (* false *)
  Printf.printf "contains a = %b\n" (contains cache "a");  (* true *)
  Printf.printf "size = %d\n" (size cache);

  (* Update existing key — should not grow cache *)
  put cache "a" 99;
  Printf.printf "get a after update = %d\n" (Option.get (get cache "a"));
  Printf.printf "size after update = %d\n" (size cache)
