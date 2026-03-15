(* 1037: Adjacency List Graph
   OCaml's Hashtbl serves as the map; lists as neighbor sets.
   BFS, DFS, and path-finding with idiomatic OCaml. *)

module IntMap = Map.Make(Int)
module IntSet = Set.Make(Int)

type graph = { adj : int list IntMap.t }

let empty = { adj = IntMap.empty }

let add_edge g from_ to_ =
  let update map key value =
    let current = try IntMap.find key map with Not_found -> [] in
    IntMap.add key (current @ [value]) map
  in
  (* Ensure both nodes exist *)
  let adj = update g.adj from_ to_ in
  let adj = if IntMap.mem to_ adj then adj
            else IntMap.add to_ [] adj in
  { adj }

let neighbors g node =
  try IntMap.find node g.adj with Not_found -> []

(* BFS traversal — returns visited order *)
let bfs g start =
  let visited = ref IntSet.empty in
  let queue = Queue.create () in
  let order = ref [] in
  visited := IntSet.add start !visited;
  Queue.add start queue;
  while not (Queue.is_empty queue) do
    let node = Queue.pop queue in
    order := node :: !order;
    List.iter (fun nb ->
      if not (IntSet.mem nb !visited) then begin
        visited := IntSet.add nb !visited;
        Queue.add nb queue
      end) (neighbors g node)
  done;
  List.rev !order

(* DFS traversal — recursive *)
let dfs g start =
  let visited = ref IntSet.empty in
  let order = ref [] in
  let rec go node =
    if not (IntSet.mem node !visited) then begin
      visited := IntSet.add node !visited;
      order := node :: !order;
      List.iter go (neighbors g node)
    end
  in
  go start;
  List.rev !order

(* BFS shortest path — returns Some path or None *)
let find_path g start goal =
  let visited = ref IntSet.empty in
  let parent = ref IntMap.empty in
  let queue = Queue.create () in
  visited := IntSet.add start !visited;
  Queue.add start queue;
  let found = ref false in
  while not (Queue.is_empty queue) && not !found do
    let node = Queue.pop queue in
    if node = goal then found := true
    else
      List.iter (fun nb ->
        if not (IntSet.mem nb !visited) then begin
          visited := IntSet.add nb !visited;
          parent := IntMap.add nb node !parent;
          Queue.add nb queue
        end) (neighbors g node)
  done;
  if not !found then None
  else begin
    (* Reconstruct path *)
    let path = ref [goal] in
    let cur = ref goal in
    while !cur <> start do
      let p = IntMap.find !cur !parent in
      path := p :: !path;
      cur := p
    done;
    Some !path
  end

let () =
  let g = ref empty in
  g := add_edge !g 0 1;
  g := add_edge !g 0 2;
  g := add_edge !g 1 3;
  g := add_edge !g 2 3;
  g := add_edge !g 3 4;

  let bfs_order = bfs !g 0 in
  assert (List.length bfs_order = 5);
  assert (List.hd bfs_order = 0);
  assert (List.mem 4 bfs_order);

  let dfs_order = dfs !g 0 in
  assert (List.length dfs_order = 5);
  assert (List.hd dfs_order = 0);

  let path = find_path !g 0 4 in
  (match path with
   | None -> assert false
   | Some p ->
     assert (List.hd p = 0);
     assert (List.nth p (List.length p - 1) = 4);
     assert (List.length p <= 4));

  (* No path back (directed graph) *)
  assert (find_path !g 4 0 = None);

  (* Disconnected graph *)
  let g2 = ref empty in
  g2 := add_edge !g2 0 1;
  g2 := add_edge !g2 2 3;
  let reachable = bfs !g2 0 in
  assert (List.length reachable = 2);
  assert (not (List.mem 2 reachable));

  Printf.printf "All adjacency-list tests passed.\n"
