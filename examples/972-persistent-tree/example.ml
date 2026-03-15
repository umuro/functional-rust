(* 972: Persistent BST (Immutable Red-Black Tree)
   OCaml's standard Map module is a balanced persistent BST.
   Each insertion/deletion returns a new tree; old versions remain valid.
   We demonstrate both the stdlib Map and a hand-rolled persistent AVL tree. *)

(* --- Using OCaml's stdlib Map (red-black tree internally) --- *)
module IntMap = Map.Make(Int)
module StringMap = Map.Make(String)

(* --- Hand-rolled persistent AVL tree --- *)
type 'a avl =
  | Empty
  | Node of { left : 'a avl; key : int; value : 'a; right : 'a avl; height : int }

let height = function Empty -> 0 | Node n -> n.height

let make_node left key value right =
  Node { left; key; value; right; height = 1 + max (height left) (height right) }

let balance_factor = function
  | Empty -> 0
  | Node n -> height n.left - height n.right

let rotate_right = function
  | Node ({ left = Node lc; _ } as n) ->
    make_node lc.left lc.key lc.value (make_node lc.right n.key n.value n.right)
  | t -> t

let rotate_left = function
  | Node ({ right = Node rc; _ } as n) ->
    make_node (make_node n.left n.key n.value rc.left) rc.key rc.value rc.right
  | t -> t

let rebalance left key value right =
  let t = make_node left key value right in
  match balance_factor t with
  | bf when bf > 1 ->
    (* Left heavy *)
    let t' = if balance_factor left < 0
             then Node { (match t with Node n -> n | _ -> assert false)
                         with left = rotate_left left }
             else t
    in rotate_right t'
  | bf when bf < -1 ->
    (* Right heavy *)
    let t' = if balance_factor right > 0
             then Node { (match t with Node n -> n | _ -> assert false)
                         with right = rotate_right right }
             else t
    in rotate_left t'
  | _ -> t

let rec insert tree k v =
  match tree with
  | Empty -> make_node Empty k v Empty
  | Node n ->
    if k < n.key      then rebalance (insert n.left k v) n.key n.value n.right
    else if k > n.key then rebalance n.left n.key n.value (insert n.right k v)
    else Node { n with value = v }  (* update existing key — O(log n) *)

let rec find tree k =
  match tree with
  | Empty -> None
  | Node n ->
    if k = n.key then Some n.value
    else if k < n.key then find n.left k
    else find n.right k

let rec to_sorted_list = function
  | Empty  -> []
  | Node n -> to_sorted_list n.left @ [(n.key, n.value)] @ to_sorted_list n.right

let () =
  (* --- stdlib Map: functional/persistent --- *)
  Printf.printf "=== stdlib Map (persistent BST) ===\n";
  let m0 = IntMap.empty in
  let m1 = IntMap.add 3 "three" m0 in
  let m2 = IntMap.add 1 "one"   m1 in
  let m3 = IntMap.add 2 "two"   m2 in

  (* All three versions are independently valid *)
  Printf.printf "m1 size=%d  m2 size=%d  m3 size=%d\n"
    (IntMap.cardinal m1) (IntMap.cardinal m2) (IntMap.cardinal m3);

  IntMap.iter (fun k v -> Printf.printf "  %d -> %s\n" k v) m3;

  let m4 = IntMap.add 2 "TWO" m3 in  (* update: returns new map *)
  Printf.printf "m3[2]=%s  m4[2]=%s  (m3 unchanged)\n"
    (IntMap.find 2 m3) (IntMap.find 2 m4);

  (* --- Hand-rolled persistent AVL tree --- *)
  Printf.printf "\n=== Persistent AVL tree ===\n";
  let t0 = Empty in
  let t1 = insert t0 5 "five" in
  let t2 = insert t1 3 "three" in
  let t3 = insert t2 7 "seven" in
  let t4 = insert t3 1 "one" in
  let t5 = insert t4 4 "four" in

  Printf.printf "t5 sorted: %s\n"
    (String.concat "; "
      (List.map (fun (k,v) -> Printf.sprintf "%d:%s" k v) (to_sorted_list t5)));

  (* Branching: t5a and t5b diverge from t4 *)
  let t5a = insert t4 4 "FOUR"  in  (* version A *)
  let t5b = insert t4 4 "vier"  in  (* version B *)
  Printf.printf "t5a[4]=%s  t5b[4]=%s  t4[4]=%s\n"
    (Option.value (find t5a 4) ~default:"None")
    (Option.value (find t5b 4) ~default:"None")
    (Option.value (find t4  4) ~default:"None");

  Printf.printf "height t5=%d (balanced)\n" (height t5)
