(* 1035: Doubly-Linked List
   OCaml uses mutable records with option references for doubly-linked lists.
   No Rc<RefCell<>> needed — OCaml's GC handles cycles safely. *)

type 'a dnode = {
  mutable value : 'a;
  mutable prev  : 'a dnode option;
  mutable next  : 'a dnode option;
}

type 'a dlist = {
  mutable head : 'a dnode option;
  mutable tail : 'a dnode option;
  mutable len  : int;
}

let make_dlist () = { head = None; tail = None; len = 0 }

let push_back dl value =
  let node = { value; prev = dl.tail; next = None } in
  (match dl.tail with
   | Some t -> t.next <- Some node
   | None   -> dl.head <- Some node);
  dl.tail <- Some node;
  dl.len <- dl.len + 1

let push_front dl value =
  let node = { value; prev = None; next = dl.head } in
  (match dl.head with
   | Some h -> h.prev <- Some node
   | None   -> dl.tail <- Some node);
  dl.head <- Some node;
  dl.len <- dl.len + 1

let pop_front dl =
  match dl.head with
  | None -> None
  | Some node ->
    (match node.next with
     | Some next -> next.prev <- None; dl.head <- Some next
     | None      -> dl.head <- None; dl.tail <- None);
    dl.len <- dl.len - 1;
    Some node.value

let pop_back dl =
  match dl.tail with
  | None -> None
  | Some node ->
    (match node.prev with
     | Some prev -> prev.next <- None; dl.tail <- Some prev
     | None      -> dl.head <- None; dl.tail <- None);
    dl.len <- dl.len - 1;
    Some node.value

(* Forward traversal via fold *)
let fold_forward f acc dl =
  let rec aux acc = function
    | None -> acc
    | Some node -> aux (f acc node.value) node.next
  in
  aux acc dl.head

let to_list dl =
  fold_forward (fun acc x -> acc @ [x]) [] dl

(* Backward traversal: start at tail, follow prev links — collect in order *)
let to_list_rev dl =
  let rec aux = function
    | None -> []
    | Some node -> node.value :: aux node.prev
  in
  aux dl.tail

let () =
  let dl = make_dlist () in
  push_back dl 1;
  push_back dl 2;
  push_back dl 3;
  push_front dl 0;
  assert (to_list dl = [0; 1; 2; 3]);
  assert (dl.len = 4);

  assert (pop_front dl = Some 0);
  assert (pop_back dl = Some 3);
  assert (to_list dl = [1; 2]);

  (* Bidirectional traversal *)
  let dl2 = make_dlist () in
  List.iter (push_back dl2) [1; 2; 3; 4; 5];
  assert (to_list dl2 = [1; 2; 3; 4; 5]);
  assert (to_list_rev dl2 = [5; 4; 3; 2; 1]);

  (* Empty edge cases *)
  let empty = make_dlist () in
  assert (pop_front empty = None);
  assert (pop_back empty = None);

  (* Single element *)
  let single = make_dlist () in
  push_back single 42;
  assert (pop_front single = Some 42);
  assert (single.head = None);
  assert (single.tail = None);

  Printf.printf "All doubly-linked list tests passed.\n"
