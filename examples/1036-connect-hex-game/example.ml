(* Connect (Hex Game) *)
(* Flood-fill / BFS on hex grid to detect connected paths *)

type player = O | X

let connect board =
  let rows = Array.of_list board in
  let h = Array.length rows in
  if h = 0 then None else
  let parse r = String.to_seq rows.(r) |> Seq.filter (fun c -> c <> ' ')
    |> Array.of_seq in
  let grid = Array.init h parse in
  let w = Array.length grid.(0) in
  let deltas = [(-1,0);(-1,1);(0,-1);(0,1);(1,-1);(1,0)] in
  let bfs start_cells goal ch =
    let visited = Array.init h (fun _ -> Array.make w false) in
    let queue = Queue.create () in
    List.iter (fun (r, c) ->
      if grid.(r).(c) = ch then (visited.(r).(c) <- true; Queue.add (r, c) queue)
    ) start_cells;
    let found = ref false in
    while not (Queue.is_empty queue) && not !found do
      let (r, c) = Queue.pop queue in
      if goal r c then found := true
      else List.iter (fun (dr, dc) ->
        let r' = r + dr and c' = c + dc in
        if r' >= 0 && r' < h && c' >= 0 && c' < w &&
           not visited.(r').(c') && grid.(r').(c') = ch then
          (visited.(r').(c') <- true; Queue.add (r', c') queue)
      ) deltas
    done; !found
  in
  let x_start = List.init h (fun r -> (r, 0)) in
  if bfs x_start (fun _ c -> c = w - 1) 'X' then Some X
  else
    let o_start = List.init w (fun c -> (0, c)) in
    if bfs o_start (fun r _ -> r = h - 1) 'O' then Some O
    else None
