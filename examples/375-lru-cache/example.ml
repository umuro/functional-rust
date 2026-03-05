(* OCaml: LRU with Hashtbl + doubly-linked list (simplified) *)

type ('k,'v) lru = {
  mutable data: ('k*'v) list;  (* Most-recent first *)
  capacity: int;
}

let make cap = { data=[]; capacity=cap }

let get lru k =
  match List.assoc_opt k lru.data with
  | None -> None
  | Some v ->
    lru.data <- (k,v) :: List.filter (fun (k2,_) -> k2 <> k) lru.data;
    Some v

let put lru k v =
  let without_k = List.filter (fun (k2,_) -> k2 <> k) lru.data in
  let with_k = (k,v) :: without_k in
  lru.data <- if List.length with_k > lru.capacity then
    List.filteri (fun i _ -> i < lru.capacity) with_k
  else with_k

let () =
  let c = make 3 in
  put c "a" 1; put c "b" 2; put c "c" 3;
  ignore (get c "a");  (* a is now most recent *)
  put c "d" 4;  (* evicts b (lru) *)
  Printf.printf "a: %s\n" (Option.fold ~none:"evicted" ~some:string_of_int (get c "a"));
  Printf.printf "b: %s\n" (Option.fold ~none:"evicted" ~some:string_of_int (get c "b"))
