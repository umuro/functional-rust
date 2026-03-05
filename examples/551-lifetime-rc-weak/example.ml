(* Rc/Weak analog in OCaml — GC handles cycles automatically *)
(* Tree with parent pointers — OCaml GC handles the cycle *)
type tree_node = {
  value: int;
  children: tree_node list ref;
  (* parent: tree_node option ref would create cycle — GC handles it *)
}

let make_node v = { value = v; children = ref [] }
let add_child parent child =
  parent.children := child :: !(parent.children)

let () =
  let root = make_node 1 in
  let child1 = make_node 2 in
  let child2 = make_node 3 in
  add_child root child1;
  add_child root child2;
  Printf.printf "root has %d children\n" (List.length !(root.children));
  List.iter (fun c -> Printf.printf "  child value: %d\n" c.value) !(root.children)
