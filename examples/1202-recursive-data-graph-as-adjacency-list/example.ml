(* Recursive Data — Graph as Adjacency List *)
(* Represent and traverse graphs using maps *)

module Graph = struct
  module SMap = Map.Make(String)
  type t = string list SMap.t

  let empty = SMap.empty
  let add_edge g u v =
    let neighbors = try SMap.find u g with Not_found -> [] in
    SMap.add u (v :: neighbors) g

  let bfs g start =
    let visited = Hashtbl.create 16 in
    let queue = Queue.create () in
    Queue.add start queue;
    Hashtbl.add visited start true;
    let result = ref [] in
    while not (Queue.is_empty queue) do
      let node = Queue.pop queue in
      result := node :: !result;
      let neighbors = try SMap.find node g with Not_found -> [] in
      List.iter (fun n ->
        if not (Hashtbl.mem visited n) then begin
          Hashtbl.add visited n true;
          Queue.add n queue
        end
      ) neighbors
    done;
    List.rev !result
end

let g = Graph.empty
  |> Graph.add_edge "A" "B" |> Graph.add_edge "A" "C"
  |> Graph.add_edge "B" "D" |> Graph.add_edge "C" "D"
let () = List.iter (fun n -> Printf.printf "%s " n) (Graph.bfs g "A")
