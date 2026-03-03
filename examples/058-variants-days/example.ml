(* Variants — Days of the Week *)

type day = Sun | Mon | Tue | Wed | Thu | Fri | Sat

(* Implementation 1: Pattern matching functions *)
let day_name = function
  | Sun -> "Sunday"    | Mon -> "Monday"  | Tue -> "Tuesday"
  | Wed -> "Wednesday" | Thu -> "Thursday" | Fri -> "Friday"
  | Sat -> "Saturday"

let is_weekend = function
  | Sun | Sat -> true
  | _         -> false

let next_day = function
  | Sun -> Mon | Mon -> Tue | Tue -> Wed | Wed -> Thu
  | Thu -> Fri | Fri -> Sat | Sat -> Sun

(* Implementation 2: Using integer encoding *)
let day_to_int = function
  | Sun -> 0 | Mon -> 1 | Tue -> 2 | Wed -> 3
  | Thu -> 4 | Fri -> 5 | Sat -> 6

let int_to_day = function
  | 0 -> Sun | 1 -> Mon | 2 -> Tue | 3 -> Wed
  | 4 -> Thu | 5 -> Fri | 6 -> Sat
  | _ -> failwith "invalid day index"

let next_day_arith d = int_to_day ((day_to_int d + 1) mod 7)

(* Tests *)
let () =
  assert (day_name Wed = "Wednesday");
  assert (is_weekend Sat = true);
  assert (is_weekend Mon = false);
  assert (next_day Sat = Sun);
  assert (next_day Wed = Thu);
  assert (next_day_arith Sat = Sun);
  (* Full cycle *)
  let d = ref Mon in
  for _ = 1 to 7 do d := next_day !d done;
  assert (!d = Mon);
  Printf.printf "All variants-days tests passed!\n"
