(* Eulerian Path/Circuit — Hierholzer's algorithm O(V+E) *)
(* Undirected graph represented as adjacency list with edge indices *)

let eulerian_path adj n =
  (* Check degrees *)
  let degree = Array.init n (fun i -> List.length adj.(i)) in
  let odd_verts = Array.to_list (Array.init n (fun i -> (i, degree.(i))))
                  |> List.filter (fun (_, d) -> d mod 2 <> 0)
                  |> List.map fst
  in
  let start = match odd_verts with
    | []    -> 0           (* circuit: start anywhere *)
    | [s;_] -> s           (* path: start at odd vertex *)
    | _     -> failwith "No Eulerian path exists (not 0 or 2 odd vertices)"
  in
  (* Mutable adjacency: array of (neighbor, used) list refs *)
  let edges = Array.init n (fun i ->
    ref (List.map (fun v -> (v, ref false)) adj.(i))
  ) in
  let stack  = ref [start] in
  let circuit = ref [] in
  while !stack <> [] do
    let v = List.hd !stack in
    (* Find first unused edge from v *)
    let rec find_edge = function
      | [] -> None
      | (u, used) :: rest ->
        if not !used then Some (u, used, rest)
        else find_edge rest
    in
    match find_edge !(edges.(v)) with
    | None ->
      circuit := v :: !circuit;
      stack   := List.tl !stack
    | Some (u, used, _) ->
      used   := true;
      (* Also mark reverse edge *)
      edges.(u) := List.map (fun (w, f) ->
        if w = v && not !f then (used := true; (w, f))  (* mark one instance *)
        else (w, f)
      ) !(edges.(u));
      stack := u :: !stack
  done;
  !circuit

let () =
  (* Simple circuit: 0-1-2-0 *)
  let n   = 3 in
  let adj = Array.make n [] in
  let add u v = adj.(u) <- v :: adj.(u); adj.(v) <- u :: adj.(v) in
  add 0 1; add 1 2; add 2 0;
  let circuit = eulerian_path adj n in
  Printf.printf "Triangle circuit: [%s]\n"
    (String.concat " -> " (List.map string_of_int circuit))
