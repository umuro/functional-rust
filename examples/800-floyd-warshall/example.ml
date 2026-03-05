(* Floyd-Warshall — all-pairs shortest paths O(V³) *)
let infinity = max_int / 2

let floyd_warshall n edges =
  let dist = Array.init n (fun i -> Array.init n (fun j ->
    if i = j then 0 else infinity
  )) in
  let next = Array.init n (fun _ -> Array.make n (-1)) in
  List.iter (fun (u, v, w) ->
    if w < dist.(u).(v) then begin
      dist.(u).(v) <- w;
      next.(u).(v) <- v
    end
  ) edges;
  for k = 0 to n - 1 do
    for i = 0 to n - 1 do
      for j = 0 to n - 1 do
        if dist.(i).(k) < infinity && dist.(k).(j) < infinity then begin
          let via = dist.(i).(k) + dist.(k).(j) in
          if via < dist.(i).(j) then begin
            dist.(i).(j) <- via;
            next.(i).(j) <- next.(i).(k)
          end
        end
      done
    done
  done;
  let neg_cycle = Array.exists (fun row -> row < 0) (Array.init n (fun i -> dist.(i).(i))) in
  (dist, next, neg_cycle)

let reconstruct next src dst =
  if next.(src).(dst) = -1 then None
  else begin
    let path = ref [src] in
    let v    = ref src in
    while !v <> dst do
      v := next.(!v).(dst);
      path := !v :: !path
    done;
    Some (List.rev !path)
  end

let () =
  let edges = [(0,1,3);(0,3,7);(1,0,8);(1,2,2);(2,0,5);(2,3,1);(3,0,2)] in
  let n = 4 in
  let (dist, next, nc) = floyd_warshall n edges in
  Printf.printf "Negative cycle: %b\n" nc;
  for i = 0 to n-1 do
    for j = 0 to n-1 do
      if dist.(i).(j) >= infinity then
        Printf.printf "  %d->%d: INF\n" i j
      else begin
        let path_str = match reconstruct next i j with
          | None   -> "none"
          | Some p -> String.concat "->" (List.map string_of_int p)
        in
        Printf.printf "  %d->%d: %d  [%s]\n" i j dist.(i).(j) path_str
      end
    done
  done
