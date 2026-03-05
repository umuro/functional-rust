(* Minimum Vertex Cover — 2-approximation O(V+E) *)

let vertex_cover_2approx n edges =
  let covered = Array.make n false in
  let cover   = ref [] in
  List.iter (fun (u, v) ->
    if not covered.(u) && not covered.(v) then begin
      covered.(u) <- true;
      covered.(v) <- true;
      cover       := u :: v :: !cover
    end
  ) edges;
  List.sort_uniq compare !cover

(* Exact MVC via backtracking — exponential, for small graphs *)
let vertex_cover_exact n edges =
  let best = ref n in
  let mask = ref 0 in

  let rec solve i current_mask current_size =
    if current_size >= !best then ()
    else if i = n then begin
      (* Verify it's a valid cover *)
      let valid = List.for_all (fun (u, v) ->
        current_mask land (1 lsl u) <> 0 ||
        current_mask land (1 lsl v) <> 0
      ) edges in
      if valid && current_size < !best then begin
        best := current_size;
        mask := current_mask
      end
    end else begin
      solve (i+1) current_mask current_size;                        (* skip i *)
      solve (i+1) (current_mask lor (1 lsl i)) (current_size + 1)  (* take i *)
    end
  in
  solve 0 0 0;
  List.init n (fun i -> (i, !mask land (1 lsl i) <> 0))
  |> List.filter_map (fun (i, b) -> if b then Some i else None)

let () =
  let n     = 7 in
  let edges = [(0,1);(0,2);(1,3);(2,3);(3,4);(4,5);(4,6)] in
  let cover2 = vertex_cover_2approx n edges in
  Printf.printf "2-approx cover (%d vertices): [%s]\n"
    (List.length cover2)
    (String.concat "," (List.map string_of_int cover2));
  let cover_exact = vertex_cover_exact n edges in
  Printf.printf "Exact cover (%d vertices): [%s]\n"
    (List.length cover_exact)
    (String.concat "," (List.map string_of_int cover_exact))
