(* 961: LRU Cache *)
(* Capacity-bounded cache: evicts least recently used on overflow *)
(* OCaml: Hashtbl for O(1) lookup + Queue for LRU order *)

type ('k, 'v) lru = {
  capacity: int;
  table: ('k, 'v) Hashtbl.t;
  order: 'k Queue.t;
}

let create capacity =
  { capacity;
    table = Hashtbl.create capacity;
    order = Queue.create () }

(* Remove first occurrence of key from queue *)
let remove_from_queue q key =
  let tmp = Queue.create () in
  Queue.iter (fun k -> if k <> key then Queue.add k tmp) q;
  Queue.clear q;
  Queue.iter (fun k -> Queue.add k q) tmp

let get cache key =
  match Hashtbl.find_opt cache.table key with
  | None -> None
  | Some v ->
    (* Move key to back (most recently used) *)
    remove_from_queue cache.order key;
    Queue.add key cache.order;
    Some v

let put cache key value =
  (* If key exists, remove from order queue first *)
  if Hashtbl.mem cache.table key then
    remove_from_queue cache.order key
  else begin
    (* If at capacity, evict LRU *)
    if Hashtbl.length cache.table >= cache.capacity then begin
      let lru_key = Queue.pop cache.order in
      Hashtbl.remove cache.table lru_key
    end
  end;
  Hashtbl.replace cache.table key value;
  Queue.add key cache.order

let size cache = Hashtbl.length cache.table

let () =
  let c = create 3 in

  put c "a" 1;
  put c "b" 2;
  put c "c" 3;

  assert (get c "a" = Some 1);
  assert (get c "b" = Some 2);
  assert (size c = 3);

  (* Access "a" to make it recently used. LRU should be "c" then "b" *)
  (* After get "a": order is c, b, a (most recent) wait.. let's check *)
  (* Initial order: a, b, c *)
  (* get "a" → moves a to back: order = b, c, a *)
  (* Now insert "d" → evict "b" (front) *)
  put c "d" 4;
  assert (size c = 3);
  assert (get c "b" = None); (* "b" should be evicted *)
  assert (get c "a" = Some 1);
  assert (get c "c" = Some 3);
  assert (get c "d" = Some 4);

  (* Update existing key *)
  put c "a" 99;
  assert (get c "a" = Some 99);
  assert (size c = 3);

  Printf.printf "✓ All tests passed\n"
