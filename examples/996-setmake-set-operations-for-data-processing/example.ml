(* Set.Make — Set Operations for Data Processing *)
(* Use sets for deduplication and membership testing *)

module StringSet = Set.Make(String)

let words = ["the"; "cat"; "sat"; "on"; "the"; "mat"; "the"; "cat"]
let unique = StringSet.of_list words
let () = Printf.printf "Unique words: %d\n" (StringSet.cardinal unique)

let stopwords = StringSet.of_list ["the"; "on"; "a"; "an"]
let content_words = StringSet.diff unique stopwords
let () = StringSet.iter (fun w -> Printf.printf "%s " w) content_words
