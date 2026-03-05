(* Token tree munching concept in OCaml *)
(* Simulated via a parser combinator approach *)

type token = Int of int | Plus | Minus | Star | LParen | RParen | EOF

let tokenize s =
  let tokens = ref [] in
  String.iter (fun c -> match c with
    | '+' -> tokens := Plus :: !tokens
    | '-' -> tokens := Minus :: !tokens
    | '*' -> tokens := Star :: !tokens
    | '(' -> tokens := LParen :: !tokens
    | ')' -> tokens := RParen :: !tokens
    | ' ' -> ()
    | c when c >= '0' && c <= '9' ->
      tokens := Int (Char.code c - Char.code '0') :: !tokens
    | _ -> ()
  ) s;
  List.rev !tokens @ [EOF]

let () =
  let toks = tokenize "1 + 2 * 3" in
  Printf.printf "Tokens: %d\n" (List.length toks - 1)  (* -1 for EOF *)
