(* OCaml: ordered map via Map module *)
module SM = Map.Make(String)

let () =
  let m = SM.empty
    |> SM.add "banana" 2
    |> SM.add "apple" 5
    |> SM.add "cherry" 1
    |> SM.add "date" 3 in
  SM.iter (fun k v -> Printf.printf "%s: %d\n" k v) m;
  Printf.printf "Min: %s\n" (fst (SM.min_binding m));
  Printf.printf "Max: %s\n" (fst (SM.max_binding m))
