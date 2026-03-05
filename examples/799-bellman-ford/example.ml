(* Bellman-Ford — O(V×E) with negative cycle detection *)
let infinity = max_int / 2

let bellman_ford n edges src =
  (* edges: (u, v, w) list *)
  let dist = Array.make n infinity in
  let prev = Array.make n (-1) in
  dist.(src) <- 0;
  (* V - 1 relaxation passes *)
  for _ = 1 to n - 1 do
    List.iter (fun (u, v, w) ->
      if dist.(u) < infinity && dist.(u) + w < dist.(v) then begin
        dist.(v) <- dist.(u) + w;
        prev.(v) <- u
      end
    ) edges
  done;
  (* Check for negative cycles *)
  let neg_cycle = List.exists (fun (u, v, w) ->
    dist.(u) < infinity && dist.(u) + w < dist.(v)
  ) edges in
  (dist, prev, neg_cycle)

let reconstruct prev dst =
  let path = ref [] in
  let v    = ref dst in
  while !v >= 0 do
    path := !v :: !path;
    v    := prev.(!v)
  done;
  !path

let () =
  (* Graph with negative edges but no negative cycle *)
  let edges = [
    (0, 1, -1); (0, 2,  4);
    (1, 2,  3); (1, 3,  2); (1, 4,  2);
    (3, 2,  5); (3, 1,  1);
    (4, 3, -3)
  ] in
  let n = 5 in
  let (dist, prev, nc) = bellman_ford n edges 0 in
  Printf.printf "Source: 0, Negative cycle: %b\n" nc;
  for i = 0 to n - 1 do
    if dist.(i) = infinity then
      Printf.printf "  0 -> %d: unreachable\n" i
    else begin
      let path = reconstruct prev i in
      Printf.printf "  0 -> %d: dist=%d, path=[%s]\n" i dist.(i)
        (String.concat "->" (List.map string_of_int path))
    end
  done;

  (* Graph with negative cycle *)
  let edges2 = [(0,1,1); (1,2,-1); (2,0,-1)] in
  let (_, _, nc2) = bellman_ford 3 edges2 0 in
  Printf.printf "\nNegative cycle test: %b\n" nc2
