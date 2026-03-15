(* 974: Skip List
   A probabilistic sorted data structure with O(log n) expected search/insert/delete.
   Each node is duplicated at multiple "express lane" levels with probability p=0.5.
   OCaml: mutable doubly-linked level arrays per node. *)

let max_level = 16
let p = 0.5

type 'a node = {
  key    : int;
  mutable value : 'a;
  forward : 'a node option array;  (* forward.(i) = next node at level i *)
}

type 'a skip_list = {
  header : 'a node;          (* sentinel with key = min_int *)
  mutable level : int;       (* current max active level *)
  mutable size  : int;
}

let make_node key value levels =
  { key; value; forward = Array.make levels None }

let create () =
  let header = make_node min_int (Obj.magic ()) max_level in
  { header; level = 0; size = 0 }

(* Random level for a new node — geometric distribution *)
let random_level () =
  let lvl = ref 0 in
  while !lvl < max_level - 1 && Random.float 1.0 < p do
    incr lvl
  done;
  !lvl

let find sl key =
  let cur = ref sl.header in
  for i = sl.level downto 0 do
    let continue_ = ref true in
    while !continue_ do
      match !cur.forward.(i) with
      | Some next when next.key < key -> cur := next
      | _ -> continue_ := false
    done
  done;
  match !cur.forward.(0) with
  | Some next when next.key = key -> Some next.value
  | _ -> None

let insert sl key value =
  (* update.(i) = the rightmost node at level i whose forward should be updated *)
  let update = Array.make max_level sl.header in
  let cur = ref sl.header in
  for i = sl.level downto 0 do
    let continue_ = ref true in
    while !continue_ do
      match !cur.forward.(i) with
      | Some next when next.key < key -> cur := next
      | _ -> continue_ := false
    done;
    update.(i) <- !cur
  done;
  (* Check if key already exists *)
  (match !cur.forward.(0) with
   | Some next when next.key = key -> next.value <- value
   | _ ->
     let new_level = random_level () in
     if new_level > sl.level then begin
       for i = sl.level + 1 to new_level do update.(i) <- sl.header done;
       sl.level <- new_level
     end;
     let node = make_node key value (new_level + 1) in
     for i = 0 to new_level do
       node.forward.(i) <- update.(i).forward.(i);
       update.(i).forward.(i) <- Some node
     done;
     sl.size <- sl.size + 1)

let delete sl key =
  let update = Array.make max_level sl.header in
  let cur = ref sl.header in
  for i = sl.level downto 0 do
    let continue_ = ref true in
    while !continue_ do
      match !cur.forward.(i) with
      | Some next when next.key < key -> cur := next
      | _ -> continue_ := false
    done;
    update.(i) <- !cur
  done;
  match !cur.forward.(0) with
  | Some target when target.key = key ->
    for i = 0 to sl.level do
      if update.(i).forward.(i) <> Some target then ()
      else update.(i).forward.(i) <- target.forward.(i)
    done;
    sl.size <- sl.size - 1;
    true
  | _ -> false

let to_sorted_list sl =
  let acc = ref [] in
  let cur = ref sl.header.forward.(0) in
  while !cur <> None do
    let n = Option.get !cur in
    acc := (n.key, n.value) :: !acc;
    cur := n.forward.(0)
  done;
  List.rev !acc

let () =
  Random.self_init ();
  let sl : string skip_list = create () in

  let items = [5,"five"; 3,"three"; 8,"eight"; 1,"one"; 4,"four"; 7,"seven"; 2,"two"] in
  List.iter (fun (k,v) -> insert sl k v) items;

  Printf.printf "size = %d\n" sl.size;
  Printf.printf "sorted: [%s]\n"
    (String.concat "; "
      (List.map (fun (k,v) -> Printf.sprintf "%d:%s" k v) (to_sorted_list sl)));

  Printf.printf "find 4 = %s\n" (Option.value (find sl 4) ~default:"None");
  Printf.printf "find 9 = %s\n" (Option.value (find sl 9) ~default:"None");

  ignore (delete sl 3);
  Printf.printf "after delete 3: [%s]\n"
    (String.concat "; "
      (List.map (fun (k,_) -> string_of_int k) (to_sorted_list sl)));

  (* Range scan: walk level-0 lane *)
  Printf.printf "keys in [2,6]: ";
  let cur = ref sl.header.forward.(0) in
  while !cur <> None do
    let n = Option.get !cur in
    if n.key >= 2 && n.key <= 6 then Printf.printf "%d " n.key;
    cur := n.forward.(0)
  done;
  print_newline ()
