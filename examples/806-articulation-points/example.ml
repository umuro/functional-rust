(* Articulation Points and Bridges — O(V+E) DFS *)

let find_aps_bridges adj n =
  let disc   = Array.make n (-1) in
  let low    = Array.make n 0 in
  let is_ap  = Array.make n false in
  let timer  = ref 0 in
  let bridges = ref [] in

  let rec dfs u parent =
    disc.(u) <- !timer;
    low.(u)  <- !timer;
    incr timer;
    let children = ref 0 in
    List.iter (fun v ->
      if disc.(v) = -1 then begin
        incr children;
        dfs v u;
        low.(u) <- min low.(u) low.(v);
        (* Bridge check *)
        if low.(v) > disc.(u) then
          bridges := (u, v) :: !bridges;
        (* Articulation point check for non-root *)
        if parent <> -1 && low.(v) >= disc.(u) then
          is_ap.(u) <- true
      end else if v <> parent then
        low.(u) <- min low.(u) disc.(v)
    ) adj.(u);
    (* Root is AP iff it has >1 DFS children *)
    if parent = -1 && !children > 1 then is_ap.(u) <- true
  in
  for v = 0 to n - 1 do
    if disc.(v) = -1 then dfs v (-1)
  done;
  let aps = Array.to_list (Array.init n (fun i -> (i, is_ap.(i))))
            |> List.filter_map (fun (i, b) -> if b then Some i else None)
  in
  (aps, !bridges)

let () =
  let n   = 7 in
  let adj = Array.make n [] in
  let add u v = adj.(u) <- v :: adj.(u); adj.(v) <- u :: adj.(v) in
  add 0 1; add 1 2; add 2 0;  (* triangle 0-1-2 *)
  add 1 3; add 3 4; add 4 5; add 5 3;  (* 1-3, triangle 3-4-5 *)
  add 3 6;  (* pendant from 3 *)
  let (aps, bridges) = find_aps_bridges adj n in
  Printf.printf "Articulation points: [%s]\n"
    (String.concat "," (List.map string_of_int aps));
  Printf.printf "Bridges:\n";
  List.iter (fun (u,v) -> Printf.printf "  %d-%d\n" u v) bridges
