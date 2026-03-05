(* Dijkstra's shortest path in OCaml *)
(* Priority queue via sorted list (simple but not optimal) *)

let dijkstra adj n src =
  let inf = max_int in
  let dist = Array.make n inf in
  let visited = Array.make n false in
  dist.(src) <- 0;
  let pq = ref [(0, src)] in
  while !pq <> [] do
    let (d, u) = List.hd !pq in
    pq := List.tl !pq;
    if not visited.(u) then begin
      visited.(u) <- true;
      List.iter (fun (v, w) ->
        if dist.(u) <> inf && dist.(u) + w < dist.(v) then begin
          dist.(v) <- dist.(u) + w;
          pq := List.sort compare ((dist.(v), v) :: !pq)
        end
      ) adj.(u)
    end
  done;
  dist

let () =
  let n = 5 in
  let adj = Array.make n [] in
  let edges = [(0,1,10);(0,2,3);(1,3,2);(2,1,4);(2,3,8);(2,4,2);(3,4,5);(4,3,7)] in
  List.iter (fun (u,v,w) -> adj.(u) <- (v,w) :: adj.(u)) edges;
  let dist = dijkstra adj n 0 in
  Printf.printf "Shortest distances from vertex 0:\n";
  Array.iteri (fun v d ->
    if d = max_int then Printf.printf "  to %d: inf\n" v
    else Printf.printf "  to %d: %d\n" v d
  ) dist
