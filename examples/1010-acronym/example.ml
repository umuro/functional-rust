(* Acronym *)
(* String splitting, filtering, and mapping with Base library *)

let delimiters = [' '; '-'; '_']
let is_relevant c =
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
  List.mem c delimiters

let acronym input =
  let filtered = String.init (String.length input)
    (fun i -> let c = input.[i] in if is_relevant c then c else ' ') in
  let words = String.split_on_char ' ' filtered
    |> List.filter (fun w -> String.length w > 0) in
  let initials = List.map (fun w -> Char.uppercase_ascii w.[0]) words in
  String.init (List.length initials) (List.nth initials)
