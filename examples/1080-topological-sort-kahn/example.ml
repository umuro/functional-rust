(* Topological Sort via Kahn's Algorithm *)

module SMap = Map.Make(String)

let kahn_sort nodes edges =
  let in_deg = List.fold_left (fun m (_, b) ->
    SMap.update b (function None -> Some 1 | Some n -> Some (n+1)) m
  ) (List.fold_left (fun m n -> SMap.add n 0 m) SMap.empty nodes) edges in
  let queue = SMap.fold (fun k v acc -> if v = 0 then k :: acc else acc) in_deg [] in
  let rec go queue in_deg result =
    match queue with
    | [] -> List.rev result
    | node :: rest ->
      let out_edges = List.filter (fun (a, _) -> a = node) edges in
      let in_deg, new_queue = List.fold_left (fun (deg, q) (_, b) ->
        let d = SMap.find b deg - 1 in
        let deg = SMap.add b d deg in
        if d = 0 then (deg, b :: q) else (deg, q)
      ) (in_deg, rest) out_edges in
      go new_queue in_deg (node :: result)
  in go queue in_deg []

let () =
  let nodes = ["a";"b";"c";"d";"e"] in
  let edges = [("a","b");("a","c");("b","d");("c","d");("d","e")] in
  let result = kahn_sort nodes edges in
  assert (List.hd result = "a");
  assert (List.nth result (List.length result - 1) = "e");
  List.iter (Printf.printf "%s ") result;
  print_newline ();
  print_endline "ok"
