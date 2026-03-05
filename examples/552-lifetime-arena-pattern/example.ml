(* Arena allocation in OCaml *)
(* OCaml's GC is essentially a generational arena *)
(* Simulating with a list that holds all allocated nodes *)

type 'a arena = 'a list ref

let make_arena () = ref []

let alloc arena value =
  arena := value :: !arena;
  value  (* returns the value -- in real arena, returns pointer *)

type node = {
  id: int;
  label: string;
  mutable edges: node list;
}

let make_node arena id label =
  alloc arena { id; label; edges = [] }

let add_edge from_node to_node =
  from_node.edges <- to_node :: from_node.edges

let () =
  let arena = make_arena () in
  let n1 = make_node arena 1 "start" in
  let n2 = make_node arena 2 "middle" in
  let n3 = make_node arena 3 "end" in
  add_edge n1 n2; add_edge n2 n3; add_edge n1 n3;
  Printf.printf "n1 edges: %d\n" (List.length n1.edges);
  Printf.printf "total nodes: %d\n" (List.length !arena);
  (* "Free" the arena: *)
  arena := []
