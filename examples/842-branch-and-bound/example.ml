(* Branch and Bound — TSP in OCaml *)
(* Uses a simple lower bound: sum of two cheapest outgoing edges per node *)

let inf = max_int / 2

(* Simple lower bound for TSP: for each unvisited node, add half the sum
   of its two cheapest edges to any other unvisited node or path endpoints *)
let lower_bound (dist : int array array) (path : int list) (n : int) : int =
  let visited = Array.make n false in
  List.iter (fun v -> visited.(v) <- true) path;
  (* Current path length *)
  let path_cost = ref 0 in
  let rec sum_path = function
    | [] | [_] -> ()
    | a :: (b :: _ as rest) -> path_cost := !path_cost + dist.(a).(b); sum_path rest
  in
  sum_path (List.rev path);
  (* For unvisited nodes + endpoints: add min edge cost *)
  let bound = ref !path_cost in
  for v = 0 to n - 1 do
    if not visited.(v) || v = List.hd path || v = List.hd (List.rev path) then begin
      let edges = Array.init n (fun u ->
        if u = v then inf
        else if visited.(u) && u <> List.hd path && u <> List.hd (List.rev path) then inf
        else dist.(v).(u)
      ) in
      Array.sort compare edges;
      let e1 = edges.(0) and e2 = if Array.length edges > 1 then edges.(1) else 0 in
      if e1 < inf then bound := !bound + (e1 + e2) / 2
    end
  done;
  !bound

(* Branch and Bound TSP solver *)
let tsp_bnb (dist : int array array) : int * int list =
  let n = Array.length dist in
  let best_cost = ref inf in
  let best_path = ref [] in
  let rec bnb path visited cost =
    let path_len = List.length path in
    if path_len = n then begin
      (* Complete tour: add return to start *)
      let total = cost + dist.(List.hd path).(List.hd (List.rev path)) in
      if total < !best_cost then begin
        best_cost := total;
        best_path := List.rev path
      end
    end else begin
      for v = 0 to n - 1 do
        if not visited.(v) then begin
          let new_cost = cost + dist.(List.hd path).(v) in
          (* Simple pruning: current cost already exceeds best *)
          if new_cost < !best_cost then begin
            visited.(v) <- true;
            bnb (v :: path) visited new_cost;
            visited.(v) <- false  (* backtrack *)
          end
        end
      done
    end
  in
  let visited = Array.make n false in
  visited.(0) <- true;
  bnb [0] visited 0;
  (!best_cost, !best_path)

let () =
  (* Small 4-city example *)
  let dist = [|
    [| 0; 10; 15; 20 |];
    [| 10; 0; 35; 25 |];
    [| 15; 35; 0; 30 |];
    [| 20; 25; 30; 0 |];
  |] in
  let (cost, path) = tsp_bnb dist in
  Printf.printf "TSP optimal cost: %d\n" cost;
  Printf.printf "Tour: %s\n"
    (String.concat " -> " (List.map string_of_int path) ^ " -> 0")
