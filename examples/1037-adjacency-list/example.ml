(* 1037: Adjacency List — BFS and DFS *)
(* Graph represented as a map from node to list of neighbors *)

module IntMap = Map.Make(Int)

type graph = int list IntMap.t

let empty_graph : graph = IntMap.empty

let add_edge g from_n to_n =
  let neighbors = match IntMap.find_opt from_n g with
    | Some ns -> ns
    | None -> []
  in
  IntMap.add from_n (to_n :: neighbors) g

let neighbors g node =
  match IntMap.find_opt node g with
  | Some ns -> ns
  | None -> []

(* Approach 1: BFS *)
let bfs g start =
  let visited = Hashtbl.create 16 in
  let queue = Queue.create () in
  let order = ref [] in
  Queue.push start queue;
  Hashtbl.add visited start true;
  while not (Queue.is_empty queue) do
    let node = Queue.pop queue in
    order := node :: !order;
    List.iter (fun neighbor ->
      if not (Hashtbl.mem visited neighbor) then begin
        Hashtbl.add visited neighbor true;
        Queue.push neighbor queue
      end
    ) (neighbors g node)
  done;
  List.rev !order

(* Approach 2: DFS (recursive) *)
let dfs g start =
  let visited = Hashtbl.create 16 in
  let order = ref [] in
  let rec visit node =
    if not (Hashtbl.mem visited node) then begin
      Hashtbl.add visited node true;
      order := node :: !order;
      List.iter visit (neighbors g node)
    end
  in
  visit start;
  List.rev !order

(* Approach 3: Path finding with BFS *)
let find_path g start goal =
  let visited = Hashtbl.create 16 in
  let parent = Hashtbl.create 16 in
  let queue = Queue.create () in
  Queue.push start queue;
  Hashtbl.add visited start true;
  let found = ref false in
  while not (Queue.is_empty queue) && not !found do
    let node = Queue.pop queue in
    if node = goal then found := true
    else
      List.iter (fun neighbor ->
        if not (Hashtbl.mem visited neighbor) then begin
          Hashtbl.add visited neighbor true;
          Hashtbl.add parent neighbor node;
          Queue.push neighbor queue
        end
      ) (neighbors g node)
  done;
  if not !found then None
  else begin
    let rec build_path node acc =
      if node = start then start :: acc
      else build_path (Hashtbl.find parent node) (node :: acc)
    in
    Some (build_path goal [])
  end

let () =
  (* Build graph: 0->1, 0->2, 1->3, 2->3, 3->4 *)
  let g = empty_graph
    |> fun g -> add_edge g 0 1
    |> fun g -> add_edge g 0 2
    |> fun g -> add_edge g 1 3
    |> fun g -> add_edge g 2 3
    |> fun g -> add_edge g 3 4
  in
  let bfs_order = bfs g 0 in
  assert (List.mem 0 bfs_order);
  assert (List.mem 4 bfs_order);
  assert (List.length bfs_order = 5);

  let dfs_order = dfs g 0 in
  assert (List.hd dfs_order = 0);
  assert (List.length dfs_order = 5);

  let path = find_path g 0 4 in
  assert (path <> None);
  let path = Option.get path in
  assert (List.hd path = 0);
  assert (List.nth path (List.length path - 1) = 4);

  Printf.printf "✓ All tests passed\n"
