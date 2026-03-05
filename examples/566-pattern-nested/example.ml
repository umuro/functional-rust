(* Nested pattern matching *)
type suit = C | D | H | S
type rank = N of int | J | Q | K | A
type card = { r: rank; s: suit }
type hand = Empty | One of card | Two of card * card

let hand_pts = function
  | Empty -> 0
  | One c -> (match c.r with N n->n | J|Q|K->10 | A->11)
  | Two(a,b) ->
    let rv = function N n->n | J|Q|K->10 | A->11 in
    rv a.r + rv b.r

let describe = function
  | One { r=A; s=S } -> "ace of spades!"
  | One { r=A; _ }   -> "an ace"
  | Two({ r=r1;_},{r=r2;_}) when r1=r2 -> "a pair"
  | Two _             -> "two cards"
  | Empty             -> "nothing"

let () =
  let h1 = One {r=A;s=S} in
  let h2 = Two({r=K;s=H},{r=K;s=C}) in
  Printf.printf "%s (%d pts)\n" (describe h1) (hand_pts h1);
  Printf.printf "%s (%d pts)\n" (describe h2) (hand_pts h2)
