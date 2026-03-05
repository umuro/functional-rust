(* OCaml: graph as adjacency list *)

module G = Map.Make(Int)

let add_edge g u v =
  let lu = try G.find u g with Not_found -> [] in
  let lv = try G.find v g with Not_found -> [] in
  g |> G.add u (v::lu) |> G.add v (u::lv)

let bfs g start =
  let visited = Hashtbl.create 8 in
  let queue = Queue.create () in
  Queue.add start queue;
  Hashtbl.add visited start true;
  let order = ref [] in
  while not (Queue.is_empty queue) do
    let node = Queue.pop queue in
    order := node :: !order;
    List.iter (fun nb ->
      if not (Hashtbl.mem visited nb) then begin
        Hashtbl.add visited nb true;
        Queue.add nb queue
      end
    ) (try G.find node g with Not_found -> [])
  done;
  List.rev !order

let () =
  let g = List.fold_left (fun g (u,v) -> add_edge g u v) G.empty
    [(0,1);(0,2);(1,3);(2,3);(3,4)] in
  Printf.printf "BFS from 0: [%s]\n"
    (String.concat ";" (List.map string_of_int (bfs g 0)))
