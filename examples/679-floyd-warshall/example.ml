(* Floyd-Warshall in OCaml *)
let floyd_warshall n edges =
  let inf = max_int / 2 in
  let dist = Array.make_matrix n n inf in
  for i = 0 to n - 1 do dist.(i).(i) <- 0 done;
  List.iter (fun (u, v, w) -> dist.(u).(v) <- w) edges;
  for k = 0 to n - 1 do
    for i = 0 to n - 1 do
      for j = 0 to n - 1 do
        dist.(i).(j) <- min dist.(i).(j) (dist.(i).(k) + dist.(k).(j))
      done
    done
  done;
  dist
