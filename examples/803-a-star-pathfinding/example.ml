(* A* Pathfinding on a grid — Manhattan heuristic *)
(* grid: '.' = open, '#' = wall, 'S' = start, 'E' = end *)

module PQ = Set.Make(struct
  type t = int * int * (int * int)  (* (f, g, pos) *)
  let compare (f1,g1,(r1,c1)) (f2,g2,(r2,c2)) =
    let cf = compare f1 f2 in if cf <> 0 then cf
    else let cg = compare g1 g2 in if cg <> 0 then cg
    else compare (r1,c1) (r2,c2)
end)

let a_star grid start goal =
  let rows = Array.length grid in
  let cols = Array.length grid.(0) in
  let inf  = max_int / 2 in
  let g_cost = Array.init rows (fun _ -> Array.make cols inf) in
  let came  = Array.init rows (fun _ -> Array.make cols None) in
  let (sr, sc) = start and (er, ec) = goal in
  let h r c = abs (r - er) + abs (c - ec) in
  g_cost.(sr).(sc) <- 0;
  let open_set = ref (PQ.singleton (h sr sc, 0, start)) in
  let found    = ref false in
  while not (PQ.is_empty !open_set) && not !found do
    let (_, g, (r, c)) = PQ.min_elt !open_set in
    open_set := PQ.remove (PQ.min_elt !open_set) !open_set;
    if g > g_cost.(r).(c) then ()  (* stale entry *)
    else if (r,c) = goal then found := true
    else List.iter (fun (dr, dc) ->
      let nr = r + dr and nc = c + dc in
      if nr >= 0 && nr < rows && nc >= 0 && nc < cols
         && grid.(nr).(nc) <> '#' then begin
        let ng = g + 1 in
        if ng < g_cost.(nr).(nc) then begin
          g_cost.(nr).(nc) <- ng;
          came.(nr).(nc)   <- Some (r, c);
          open_set := PQ.add (ng + h nr nc, ng, (nr, nc)) !open_set
        end
      end
    ) [(-1,0);(1,0);(0,-1);(0,1)]
  done;
  if not !found then None
  else begin
    let path = ref [] in
    let pos  = ref goal in
    while !pos <> start do
      path := !pos :: !path;
      let (r,c) = !pos in
      pos := match came.(r).(c) with Some p -> p | None -> start
    done;
    path := start :: !path;
    Some !path
  end

let () =
  let grid = [|
    [|'.';'.';'.';'#';'.'|];
    [|'.';'#';'.';'#';'.'|];
    [|'.';'#';'.';'.';'.'|];
    [|'.';'.';'#';'#';'.'|];
    [|'#';'.';'.';'.';'.'|];
  |] in
  match a_star grid (0,0) (4,4) with
  | None   -> Printf.printf "No path found\n"
  | Some p ->
    Printf.printf "Path length: %d\n" (List.length p - 1);
    Printf.printf "Path: %s\n"
      (String.concat " -> " (List.map (fun (r,c) -> Printf.sprintf "(%d,%d)" r c) p))
