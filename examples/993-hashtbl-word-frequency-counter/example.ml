(* Hashtbl — Word Frequency Counter *)
(* Use Hashtbl to count word occurrences *)

let count_words text =
  let tbl = Hashtbl.create 32 in
  let words = String.split_on_char ' ' text in
  List.iter (fun w ->
    let w = String.lowercase_ascii w in
    let n = try Hashtbl.find tbl w with Not_found -> 0 in
    Hashtbl.replace tbl w (n + 1)
  ) words;
  tbl

let freq = count_words "the cat sat on the mat the cat"
let () = Hashtbl.iter (fun w n -> Printf.printf "%s: %d\n" w n) freq
