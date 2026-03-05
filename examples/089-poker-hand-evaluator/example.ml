(* Poker Hand Evaluator — Complex Pattern Matching *)

type rank = int
type hand_type = HighCard | Pair | TwoPair | ThreeKind | Straight
  | Flush | FullHouse | FourKind | StraightFlush

(* Idiomatic OCaml — classify a 5-card hand given ranks and flush status *)
let classify (ranks : rank list) (is_flush : bool) =
  let sorted = List.sort (fun a b -> compare b a) ranks in
  let counts = List.sort (fun a b -> compare b a)
    (List.map (fun r -> List.length (List.filter ((=) r) sorted))
      (List.sort_uniq compare sorted)) in
  let is_straight = match sorted with
    | [a;_;_;_;e] -> a - e = 4 && List.length (List.sort_uniq compare sorted) = 5
    | _ -> false in
  match is_flush, is_straight, counts with
  | true, true, _ -> StraightFlush
  | _, _, 4 :: _ -> FourKind
  | _, _, [3; 2] -> FullHouse
  | true, _, _ -> Flush
  | _, true, _ -> Straight
  | _, _, 3 :: _ -> ThreeKind
  | _, _, [2; 2; 1] -> TwoPair
  | _, _, 2 :: _ -> Pair
  | _ -> HighCard

let name = function
  | StraightFlush -> "Straight Flush" | FourKind -> "Four of a Kind"
  | FullHouse -> "Full House" | Flush -> "Flush" | Straight -> "Straight"
  | ThreeKind -> "Three of a Kind" | TwoPair -> "Two Pair"
  | Pair -> "Pair" | HighCard -> "High Card"

(* Recursive helper — count occurrences of a value in a list *)
let rec count x = function
  | [] -> 0
  | h :: t -> (if h = x then 1 else 0) + count x t

(* Recursive version — builds counts by explicit recursion over unique ranks *)
let rec unique = function
  | [] -> []
  | h :: t -> h :: unique (List.filter ((<>) h) t)

let classify_recursive (ranks : rank list) (is_flush : bool) =
  let sorted = List.sort (fun a b -> compare b a) ranks in
  let uniq = List.sort compare (unique sorted) in
  let counts = List.sort (fun a b -> compare b a) (List.map (fun r -> count r sorted) uniq) in
  let is_straight = match sorted with
    | [a;_;_;_;e] -> a - e = 4 && List.length uniq = 5
    | _ -> false in
  match is_flush, is_straight, counts with
  | true, true, _ -> StraightFlush
  | _, _, 4 :: _ -> FourKind
  | _, _, [3; 2] -> FullHouse
  | true, _, _ -> Flush
  | _, true, _ -> Straight
  | _, _, 3 :: _ -> ThreeKind
  | _, _, [2; 2; 1] -> TwoPair
  | _, _, 2 :: _ -> Pair
  | _ -> HighCard

let () =
  assert (classify [10;11;12;13;14] true  = StraightFlush);
  assert (classify [2;3;4;5;6]     true  = StraightFlush);
  assert (classify [9;9;9;9;5]     false = FourKind);
  assert (classify [3;3;3;7;7]     false = FullHouse);
  assert (classify [2;5;7;9;11]    true  = Flush);
  assert (classify [5;6;7;8;9]     false = Straight);
  assert (classify [8;8;8;3;5]     false = ThreeKind);
  assert (classify [4;4;7;7;9]     false = TwoPair);
  assert (classify [2;2;5;8;11]    false = Pair);
  assert (classify [2;5;9;11;14]   false = HighCard);
  assert (classify_recursive [3;3;3;7;7] false = FullHouse);
  Printf.printf "%s\n" (name (classify [10;11;12;13;14] true));
  Printf.printf "%s\n" (name (classify [3;3;3;7;7] false));
  print_endline "ok"
