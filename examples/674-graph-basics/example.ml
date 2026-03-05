(* Graph basics in OCaml *)
module IntMap = Map.Make(Int)

let () =
  let g = IntMap.empty
    |> IntMap.add 1 [2; 3]
    |> IntMap.add 2 [4] in
  match IntMap.find_opt 1 g with
  | Some ns -> Printf.printf "Neighbors of 1: [%s]\n"
      (String.concat "; " (List.map string_of_int ns))
  | None -> ()
