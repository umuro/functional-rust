(* String Processing — Simple Lexer *)
(* Tokenize a string into a list of tokens *)

type token = TInt of int | TOp of char | TLParen | TRParen

let is_digit c = c >= '0' && c <= '9'
let is_op c = c = '+' || c = '-' || c = '*' || c = '/'

let tokenize s =
  let n = String.length s in
  let rec aux i acc =
    if i >= n then List.rev acc
    else if s.[i] = ' ' then aux (i+1) acc
    else if s.[i] = '(' then aux (i+1) (TLParen :: acc)
    else if s.[i] = ')' then aux (i+1) (TRParen :: acc)
    else if is_op s.[i] then aux (i+1) (TOp s.[i] :: acc)
    else if is_digit s.[i] then
      let j = ref i in
      while !j < n && is_digit s.[!j] do incr j done;
      aux !j (TInt (int_of_string (String.sub s i (!j - i))) :: acc)
    else failwith (Printf.sprintf "unexpected: %c" s.[i])
  in aux 0 []

let tokens = tokenize "(42 + 3) * 7"
let () = Printf.printf "%d tokens\n" (List.length tokens)
