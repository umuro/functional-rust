(* Idiomatic OCaml BFS: Hashtbl for visited, Queue for frontier *)
let bfs graph start =
  let visited = Hashtbl.create 16 in
  let queue = Queue.create () in
  Queue.push start queue;
  Hashtbl.add visited start true;
  let result = ref [] in
  while not (Queue.is_empty queue) do
    let node = Queue.pop queue in
    result := node :: !result;
    List.iter (fun neighbor ->
      if not (Hashtbl.mem visited neighbor) then begin
        Hashtbl.add visited neighbor true;
        Queue.push neighbor queue
      end
    ) (List.assoc node graph)
  done;
  List.rev !result

(* Purely functional BFS using list-based queue — shows explicit recursion *)
let bfs_pure graph start =
  let rec loop visited queue result =
    match queue with
    | [] -> List.rev result
    | node :: rest ->
      let neighbors = List.assoc node graph in
      let new_nodes = List.filter (fun n -> not (List.mem n visited)) neighbors in
      loop (visited @ new_nodes) (rest @ new_nodes) (node :: result)
  in
  loop [start] [start] []

let () =
  let graph = [("a", ["b";"c"]); ("b", ["d"]); ("c", ["d"]); ("d", [])] in
  let order = bfs graph "a" in
  assert (List.hd order = "a");
  assert (List.length order = 4);
  assert (List.mem "d" order);
  let order2 = bfs_pure graph "a" in
  assert (List.hd order2 = "a");
  assert (List.length order2 = 4);
  print_endline "ok"
