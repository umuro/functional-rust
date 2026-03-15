(* 1068: Maze Solver — DFS Backtracking and BFS on 2D Grid
   0 = open, 1 = wall. Find path from start to end. *)

let dirs = [|(0,1);(1,0);(0,-1);(-1,0)|]

(* Approach 1: DFS backtracking — finds any path *)
let solve_maze maze start end_ =
  let rows = Array.length maze in
  let cols = Array.length maze.(0) in
  let visited = Array.init rows (fun _ -> Array.make cols false) in
  let path = ref [] in
  let found = ref false in
  let rec dfs r c =
    if !found then ()
    else if (r, c) = end_ then begin
      path := (r, c) :: !path;
      found := true
    end else if maze.(r).(c) = 1 || visited.(r).(c) then ()
    else begin
      visited.(r).(c) <- true;
      path := (r, c) :: !path;
      Array.iter (fun (dr, dc) ->
        let nr = r + dr and nc = c + dc in
        if nr >= 0 && nr < rows && nc >= 0 && nc < cols then
          dfs nr nc
      ) dirs;
      if not !found then path := List.tl !path
    end
  in
  dfs (fst start) (snd start);
  if !found then Some (List.rev !path) else None

(* Approach 2: BFS — finds shortest path *)
let solve_maze_bfs maze start end_ =
  let rows = Array.length maze in
  let cols = Array.length maze.(0) in
  let visited = Array.init rows (fun _ -> Array.make cols false) in
  let parent = Array.init rows (fun _ -> Array.make cols (-1, -1)) in
  let q = Queue.create () in
  visited.(fst start).(snd start) <- true;
  Queue.add start q;
  let found = ref false in
  while not (Queue.is_empty q) && not !found do
    let (r, c) = Queue.pop q in
    if (r, c) = end_ then found := true
    else
      Array.iter (fun (dr, dc) ->
        let nr = r + dr and nc = c + dc in
        if nr >= 0 && nr < rows && nc >= 0 && nc < cols
           && maze.(nr).(nc) = 0 && not visited.(nr).(nc) then begin
          visited.(nr).(nc) <- true;
          parent.(nr).(nc) <- (r, c);
          Queue.add (nr, nc) q
        end
      ) dirs
  done;
  if not !found then None
  else begin
    let path = ref [end_] in
    let (r, c) = ref (fst end_), ref (snd end_) in
    while (!r, !c) <> start do
      let (pr, pc) = parent.(!r).(!c) in
      path := (pr, pc) :: !path;
      r := pr; c := pc
    done;
    Some !path
  end

let test_maze () =
  [| [|0;0;1;0;0|];
     [|0;0;0;0;1|];
     [|1;1;0;1;0|];
     [|0;0;0;0;0|];
     [|0;1;1;0;0|] |]

let () =
  let maze = test_maze () in

  (match solve_maze maze (0,0) (4,4) with
   | None -> assert false
   | Some path ->
     assert (List.hd path = (0,0));
     assert (List.nth path (List.length path - 1) = (4,4)));

  (match solve_maze_bfs maze (0,0) (4,4) with
   | None -> assert false
   | Some path ->
     assert (List.hd path = (0,0));
     assert (List.nth path (List.length path - 1) = (4,4)));

  (* Impossible maze *)
  let impossible = [| [|0;1|]; [|1;0|] |] in
  assert (solve_maze impossible (0,0) (1,1) = None);

  Printf.printf "All maze-solver tests passed.\n"
