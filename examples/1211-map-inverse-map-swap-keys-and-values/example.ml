(* Map — Inverse Map (Swap Keys and Values) *)
(* Create an inverse mapping from values to keys *)

module SMap = Map.Make(String)
module IMap = Map.Make(Int)

let invert_map m =
  SMap.fold (fun k v acc ->
    let keys = match IMap.find_opt v acc with
      | Some ks -> k :: ks | None -> [k]
    in IMap.add v keys acc
  ) m IMap.empty

let scores = SMap.of_list [("Alice", 95); ("Bob", 87); ("Carol", 95); ("Dave", 87)]
let by_score = invert_map scores

let () = IMap.iter (fun score names ->
  Printf.printf "Score %d: %s\n" score (String.concat ", " names)
) by_score
