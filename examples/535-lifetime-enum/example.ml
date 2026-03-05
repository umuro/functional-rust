(* Enums with string references in OCaml — GC handles lifetimes *)
type token =
  | Word of string
  | Number of int
  | Punctuation of char
  | End

type parse_result =
  | Ok of token * string   (* token and remaining input *)
  | Error of string * int  (* message and position *)

let parse_token input pos =
  if pos >= String.length input then Ok (End, "")
  else
    let c = input.[pos] in
    if c >= '0' && c <= '9' then
      Ok (Number (Char.code c - Char.code '0'), String.sub input (pos+1) (String.length input - pos - 1))
    else if c = ' ' then
      Ok (Punctuation ' ', String.sub input (pos+1) (String.length input - pos - 1))
    else
      Ok (Word (String.make 1 c), String.sub input (pos+1) (String.length input - pos - 1))

let () =
  let input = "hello 42" in
  let rec go pos =
    if pos >= String.length input then ()
    else match parse_token input pos with
    | Ok (End, _) -> ()
    | Ok (Word w, _rest) -> Printf.printf "Word: %s\n" w; go (pos + 1)
    | Ok (Number n, _rest) -> Printf.printf "Number: %d\n" n; go (pos + 1)
    | Ok (Punctuation c, _rest) -> Printf.printf "Punc: %c\n" c; go (pos + 1)
    | Error (msg, p) -> Printf.printf "Error at %d: %s\n" p msg
  in
  go 0
