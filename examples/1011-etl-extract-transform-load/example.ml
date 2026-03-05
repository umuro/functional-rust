(* ETL (Extract-Transform-Load) *)
(* Data transformation — inverting key-value pairs *)

let transform data =
  let assign points letter = (Char.lowercase_ascii letter, points) in
  let gather (points, letters) = List.map (assign points) letters in
  let compare (a, _) (b, _) = Char.compare a b in
  List.sort compare (List.concat_map gather data)
