(* 1068: Maze Solver — Backtracking on 2D Grid *)

(* 0 = open, 1 = wall *)
let directions = [| (0, 1); (1, 0); (0, -1); (-1, 0) |]

(* Approach 1: Backtracking to find any path *)
let solve_maze maze start_r start_c end_r end_c =
  let rows = Array.length maze in
  let cols = Array.length maze.(0) in
  let visited = Array.init rows (fun _ -> Array.make cols false) in
  let path = ref [] in
  let rec dfs r c =
    if r = end_r && c = end_c then begin
      path := (r, c) :: !path;
      true
    end else if r < 0 || r >= rows || c < 0 || c >= cols
              || maze.(r).(c) = 1 || visited.(r).(c) then false
    else begin
      visited.(r).(c) <- true;
      path := (r, c) :: !path;
      let found = ref false in
      Array.iter (fun (dr, dc) ->
        if not !found then
          found := dfs (r + dr) (c + dc)
      ) directions;
      if not !found then
        path := List.tl !path;
      !found
    end
  in
  if dfs start_r start_c then Some (List.rev !path) else None

(* Approach 2: BFS for shortest path *)
let solve_maze_bfs maze start_r start_c end_r end_c =
  let rows = Array.length maze in
  let cols = Array.length maze.(0) in
  let visited = Array.init rows (fun _ -> Array.make cols false) in
  let parent = Array.init rows (fun _ -> Array.make cols (-1, -1)) in
  let queue = Queue.create () in
  Queue.push (start_r, start_c) queue;
  visited.(start_r).(start_c) <- true;
  let found = ref false in
  while not (Queue.is_empty queue) && not !found do
    let (r, c) = Queue.pop queue in
    if r = end_r && c = end_c then found := true
    else
      Array.iter (fun (dr, dc) ->
        let nr = r + dr and nc = c + dc in
        if nr >= 0 && nr < rows && nc >= 0 && nc < cols
           && maze.(nr).(nc) = 0 && not visited.(nr).(nc) then begin
          visited.(nr).(nc) <- true;
          parent.(nr).(nc) <- (r, c);
          Queue.push (nr, nc) queue
        end
      ) directions
  done;
  if not !found then None
  else begin
    let path = ref [(end_r, end_c)] in
    let r = ref end_r and c = ref end_c in
    while (!r, !c) <> (start_r, start_c) do
      let (pr, pc) = parent.(!r).(!c) in
      path := (pr, pc) :: !path;
      r := pr; c := pc
    done;
    Some !path
  end

let () =
  let maze = [|
    [|0; 0; 1; 0; 0|];
    [|0; 0; 0; 0; 1|];
    [|1; 1; 0; 1; 0|];
    [|0; 0; 0; 0; 0|];
    [|0; 1; 1; 0; 0|]
  |] in
  (match solve_maze maze 0 0 4 4 with
   | Some path -> assert (List.hd path = (0, 0)); assert (List.nth path (List.length path - 1) = (4, 4))
   | None -> assert false);
  (match solve_maze_bfs maze 0 0 4 4 with
   | Some path -> assert (List.hd path = (0, 0))
   | None -> assert false);

  (* Impossible maze *)
  let maze2 = [| [|0; 1|]; [|1; 0|] |] in
  assert (solve_maze maze2 0 0 1 1 = None);

  Printf.printf "✓ All tests passed\n"
