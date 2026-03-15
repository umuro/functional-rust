let words = ["banana"; "apple"; "cherry"; "date"]
let sorted = List.sort String.compare words
let by_length = List.sort (fun a b -> compare (String.length a) (String.length b)) words
let descending = List.sort (fun a b -> String.compare b a) words
let () = List.iter (fun s -> Printf.printf "%s " s) sorted
