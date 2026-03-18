module PQ = Set.Make(struct
  type t = int * string
  let compare (d1,n1) (d2,n2) = compare (d1,n1) (d2,n2)
end)
module SMap = Map.Make(String)

let dijkstra graph start =
  let dist = SMap.singleton start 0 in
  let pq = PQ.singleton (0, start) in
  let rec go pq dist =
    if PQ.is_empty pq then dist
    else
      let (d, u) = PQ.min_elt pq in
      let pq = PQ.remove (d, u) pq in
      let neighbors = try SMap.find u graph with Not_found -> [] in
      let dist, pq = List.fold_left (fun (dist, pq) (v, w) ->
        let alt = d + w in
        let current = try SMap.find v dist with Not_found -> max_int in
        if alt < current then
          (SMap.add v alt dist, PQ.add (alt, v) pq)
        else (dist, pq)
      ) (dist, pq) neighbors in
      go pq dist
  in go pq dist

let () =
  let g = SMap.of_list [
    ("a",[("b",1);("c",4)]); ("b",[("c",2);("d",6)]);
    ("c",[("d",3)]); ("d",[])
  ] in
  let dist = dijkstra g "a" in
  assert (SMap.find "a" dist = 0);
  assert (SMap.find "b" dist = 1);
  assert (SMap.find "c" dist = 3); (* via b, not direct a->c=4 *)
  assert (SMap.find "d" dist = 6); (* a->b->c->d = 1+2+3 *)
  SMap.iter (Printf.printf "%s: %d\n") dist;
  print_endline "ok"