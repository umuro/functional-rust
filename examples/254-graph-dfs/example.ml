(* OCaml Depth-First Search using a pure functional visited set *)

module SS = Set.Make(String)

(* Idiomatic OCaml DFS — visited set threaded as a pure value through recursion *)
let dfs graph start =
  let rec go visited node =
    if SS.mem node visited then (visited, [])
    else
      let visited = SS.add node visited in
      let neighbors = try List.assoc node graph with Not_found -> [] in
      let visited, paths = List.fold_left (fun (vis, acc) n ->
        let vis, path = go vis n in
        (vis, acc @ path)
      ) (visited, []) neighbors in
      (visited, node :: paths)
  in
  snd (go SS.empty start)

(* Recursive DFS returning only the path — slightly more direct *)
let dfs_simple graph start =
  let rec go visited node =
    if SS.mem node visited then (visited, [])
    else
      let visited' = SS.add node visited in
      let neighbors = try List.assoc node graph with Not_found -> [] in
      let visited'', rest = List.fold_left (fun (v, acc) n ->
        let v', p = go v n in (v', acc @ p)
      ) (visited', []) neighbors in
      (visited'', node :: rest)
  in
  snd (go SS.empty start)

let () =
  let g = [("a", ["b"; "c"]); ("b", ["d"]); ("c", ["d"]); ("d", [])] in

  let result = dfs g "a" in
  assert (result = ["a"; "b"; "d"; "c"]);

  let result2 = dfs_simple g "a" in
  assert (result2 = ["a"; "b"; "d"; "c"]);

  (* Single node *)
  let single = [("x", [])] in
  assert (dfs single "x" = ["x"]);

  (* Linear chain *)
  let chain = [("1", ["2"]); ("2", ["3"]); ("3", [])] in
  assert (dfs chain "1" = ["1"; "2"; "3"]);

  print_endline "ok"
