(* Bellman-Ford in OCaml *)
let bellman_ford edges n start =
  let dist = Array.make n max_int in
  dist.(start) <- 0;
  for _ = 1 to n - 1 do
    List.iter (fun (u, v, w) ->
      if dist.(u) <> max_int && dist.(u) + w < dist.(v) then
        dist.(v) <- dist.(u) + w
    ) edges
  done;
  (* Check negative cycle *)
  if List.exists (fun (u, v, w) -> 
    dist.(u) <> max_int && dist.(u) + w < dist.(v)
  ) edges then None
  else Some dist
