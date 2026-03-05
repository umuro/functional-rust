(* Consolidating OCaml match arms *)
type token = Plus|Minus|Star|Slash|Eq|Ne|Lt|Le|Gt|Ge|LParen|RParen|Num of int|Ident of string

let token_type = function
  | Plus|Minus|Star|Slash -> "arithmetic"
  | Eq|Ne|Lt|Le|Gt|Ge    -> "comparison"
  | LParen|RParen         -> "bracket"
  | Num _                 -> "number"
  | Ident _               -> "identifier"

let precedence = function
  | Plus|Minus            -> 1
  | Star|Slash            -> 2
  | Eq|Ne|Lt|Le|Gt|Ge    -> 0
  | _                     -> -1

let () =
  let toks = [Plus;Star;Eq;Lt;LParen;Num 42;Ident"x"] in
  List.iter (fun t ->
    Printf.printf "type=%s prec=%d\n" (token_type t) (precedence t)
  ) toks
