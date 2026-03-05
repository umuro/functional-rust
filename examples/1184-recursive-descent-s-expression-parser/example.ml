(* Recursive Descent — S-Expression Parser *)
(* Parse S-expressions from a token stream *)

type sexp = Atom of string | List of sexp list

let tokenize s =
  let s = String.concat " ( " (String.split_on_char '(' s) in
  let s = String.concat " ) " (String.split_on_char ')' s) in
  String.split_on_char ' ' s |> List.filter (fun t -> t <> "")

let rec parse_sexp = function
  | [] -> failwith "unexpected end"
  | "(" :: rest ->
    let (items, rest) = parse_list rest in
    (List items, rest)
  | ")" :: _ -> failwith "unexpected )"
  | atom :: rest -> (Atom atom, rest)
and parse_list = function
  | ")" :: rest -> ([], rest)
  | tokens ->
    let (item, rest) = parse_sexp tokens in
    let (items, rest) = parse_list rest in
    (item :: items, rest)

let rec to_string = function
  | Atom s -> s
  | List l -> "(" ^ String.concat " " (List.map to_string l) ^ ")"

let (ast, _) = parse_sexp (tokenize "(define (square x) (* x x))")
let () = Printf.printf "%s\n" (to_string ast)
