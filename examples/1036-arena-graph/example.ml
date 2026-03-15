(* 1036: Graph with Arena Allocation
   OCaml uses arrays indexed by int as arenas — same pattern as Rust's Vec<Node>.
   Nodes are stored in a dynamic array (resizable via Buffer or plain arrays). *)

type node = {
  label : string;
  mutable edges : int list;  (* indices into the arena *)
}

type graph = {
  mutable nodes : node array;
  mutable size  : int;
}

let make_graph capacity =
  { nodes = Array.init capacity (fun _ -> { label = ""; edges = [] }); size = 0 }

let add_node g label =
  let idx = g.size in
  (* Grow if needed *)
  if idx >= Array.length g.nodes then begin
    let new_nodes = Array.init (idx * 2 + 1) (fun _ -> { label = ""; edges = [] }) in
    Array.blit g.nodes 0 new_nodes 0 idx;
    g.nodes <- new_nodes
  end;
  g.nodes.(idx) <- { label; edges = [] };
  g.size <- g.size + 1;
  idx

let add_edge g from_ to_ =
  let n = g.nodes.(from_) in
  g.nodes.(from_) <- { n with edges = n.edges @ [to_] }

let neighbors g idx = g.nodes.(idx).edges
let label g idx = g.nodes.(idx).label

(* BFS traversal *)
let bfs g start =
  let visited = Array.make g.size false in
  let queue = Queue.create () in
  let order = ref [] in
  visited.(start) <- true;
  Queue.add start queue;
  while not (Queue.is_empty queue) do
    let idx = Queue.pop queue in
    order := idx :: !order;
    List.iter (fun nb ->
      if not visited.(nb) then begin
        visited.(nb) <- true;
        Queue.add nb queue
      end) (neighbors g idx)
  done;
  List.rev !order

(* DFS traversal *)
let dfs g start =
  let visited = Array.make g.size false in
  let order = ref [] in
  let rec go idx =
    if not visited.(idx) then begin
      visited.(idx) <- true;
      order := idx :: !order;
      List.iter go (neighbors g idx)
    end
  in
  go start;
  List.rev !order

let () =
  (* Basic arena graph *)
  let g = make_graph 8 in
  let a = add_node g "A" in
  let b = add_node g "B" in
  let c = add_node g "C" in
  add_edge g a b;
  add_edge g a c;
  add_edge g b c;
  add_edge g c a;
  assert (label g a = "A");
  let nb_labels = List.map (label g) (neighbors g a) in
  assert (nb_labels = ["B"; "C"]);

  (* BFS test *)
  let g2 = make_graph 8 in
  let start = add_node g2 "start" in
  let mid1  = add_node g2 "mid1" in
  let mid2  = add_node g2 "mid2" in
  let end_  = add_node g2 "end" in
  add_edge g2 start mid1;
  add_edge g2 start mid2;
  add_edge g2 mid1 end_;
  add_edge g2 mid2 end_;
  let bfs_order = List.map (label g2) (bfs g2 start) in
  assert (bfs_order = ["start"; "mid1"; "mid2"; "end"]);

  (* DFS test *)
  let g3 = make_graph 8 in
  let na = add_node g3 "A" in
  let nb = add_node g3 "B" in
  let nc = add_node g3 "C" in
  let nd = add_node g3 "D" in
  add_edge g3 na nb;
  add_edge g3 na nc;
  add_edge g3 nb nd;
  let dfs_order = List.map (label g3) (dfs g3 na) in
  assert (dfs_order = ["A"; "B"; "D"; "C"]);

  Printf.printf "All arena-graph tests passed.\n"
