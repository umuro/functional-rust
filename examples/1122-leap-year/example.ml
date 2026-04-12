(* Idiomatic OCaml: Gregorian leap year rule *)
let is_leap_year year =
  (year mod 400 = 0) ||
    (year mod 4 = 0 && year mod 100 <> 0)

(* Explicit decomposition: name each sub-predicate *)
let is_leap_year_explicit year =
  let by4   = year mod 4   = 0 in
  let by100 = year mod 100 = 0 in
  let by400 = year mod 400 = 0 in
  by400 || (by4 && not by100)

let () =
  assert (is_leap_year 2000 = true);
  assert (is_leap_year 1900 = false);
  assert (is_leap_year 2024 = true);
  assert (is_leap_year 2023 = false);
  assert (is_leap_year 1600 = true);
  assert (is_leap_year_explicit 2000 = true);
  assert (is_leap_year_explicit 1900 = false);
  print_endline "ok"
