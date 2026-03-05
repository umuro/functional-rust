(* Interior mutability in OCaml — native refs are always interior mutable *)
type node = {
  value: int;
  mutable next: node option;
  visit_count: int ref;
}

let make_node v = { value = v; next = None; visit_count = ref 0 }

let visit node =
  node.visit_count := !(node.visit_count) + 1;
  node.value

let () =
  let n1 = make_node 10 in
  let n2 = make_node 20 in
  (* Both nodes can be "shared" and mutated via refs *)
  let _ = n2 in
  Printf.printf "n1 value: %d\n" (visit n1);
  Printf.printf "n1 value: %d\n" (visit n1);
  Printf.printf "n1 visits: %d\n" !(n1.visit_count)
