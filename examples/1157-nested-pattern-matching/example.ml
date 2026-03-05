(* Nested Pattern Matching *)
(* Complex nested patterns with guards *)

type card = { suit: string; rank: int }

let card_name c = match c.rank with
  | 1 -> "Ace of " ^ c.suit
  | 11 -> "Jack of " ^ c.suit
  | 12 -> "Queen of " ^ c.suit
  | 13 -> "King of " ^ c.suit
  | n -> string_of_int n ^ " of " ^ c.suit

let compare_hands h1 h2 = match (h1, h2) with
  | ([], []) -> 0
  | ([], _) -> -1
  | (_, []) -> 1
  | (c1 :: _, c2 :: _) when c1.rank <> c2.rank -> compare c2.rank c1.rank
  | (_ :: rest1, _ :: rest2) -> compare_hands rest1 rest2

let hand = [{ suit="Hearts"; rank=13 }; { suit="Spades"; rank=1 }]
let () = List.iter (fun c -> Printf.printf "%s\n" (card_name c)) hand
