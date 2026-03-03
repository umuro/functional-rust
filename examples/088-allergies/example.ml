(* Allergies — Bitflag Decoding *)

type allergen = Eggs | Peanuts | Shellfish | Strawberries
  | Tomatoes | Chocolate | Pollen | Cats

let allergen_score = function
  | Eggs -> 1 | Peanuts -> 2 | Shellfish -> 4 | Strawberries -> 8
  | Tomatoes -> 16 | Chocolate -> 32 | Pollen -> 64 | Cats -> 128

let all = [Eggs;Peanuts;Shellfish;Strawberries;Tomatoes;Chocolate;Pollen;Cats]

let is_allergic_to allergen score =
  score land allergen_score allergen <> 0

let allergies score =
  List.filter (fun a -> is_allergic_to a score) all

let () =
  assert (allergies 34 |> List.map allergen_score = [2; 32]);
  assert (is_allergic_to Peanuts 34);
  assert (not (is_allergic_to Eggs 34))
