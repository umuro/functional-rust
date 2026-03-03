module SMap = Map.Make(String)

let word_freq text =
  text |> String.split_on_char ' '
  |> List.map String.lowercase_ascii
  |> List.fold_left (fun acc w ->
    let count = try SMap.find w acc with Not_found -> 0 in
    SMap.add w (count + 1) acc
  ) SMap.empty

let () =
  let freq = word_freq "the cat sat on the mat the cat" in
  SMap.iter (Printf.printf "%s: %d\n") freq
