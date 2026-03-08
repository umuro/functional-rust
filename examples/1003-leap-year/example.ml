(** Leap Year Validator
    
    A year is a leap year if:
    - It is divisible by 400, OR
    - It is divisible by 4 AND not divisible by 100
*)

(** Determines if a year is a leap year.
    
    @param year the year to check
    @return true if the year is a leap year, false otherwise
*)
let is_leap_year year =
  (year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)

(** Alternative implementation using if-then-else (more imperative).
    Demonstrates a different coding style in OCaml.
*)
let is_leap_year_alt year =
  if year mod 400 = 0 then
    true
  else if year mod 100 = 0 then
    false
  else
    year mod 4 = 0

(* Test cases *)

(* Divisible by 400 - always leap *)
let () = assert (is_leap_year 2000)
let () = assert (is_leap_year 2400)
let () = assert (is_leap_year 1600)

(* Divisible by 100 but not 400 - never leap *)
let () = assert (not (is_leap_year 1900))
let () = assert (not (is_leap_year 2100))
let () = assert (not (is_leap_year 1800))

(* Divisible by 4 but not 100 - always leap *)
let () = assert (is_leap_year 2004)
let () = assert (is_leap_year 2008)
let () = assert (is_leap_year 2012)
let () = assert (is_leap_year 2016)

(* Not divisible by 4 - not leap *)
let () = assert (not (is_leap_year 2001))
let () = assert (not (is_leap_year 2002))
let () = assert (not (is_leap_year 2003))
let () = assert (not (is_leap_year 2017))

(* Both implementations match *)
let () = assert (is_leap_year 2000 = is_leap_year_alt 2000)
let () = assert (is_leap_year 1900 = is_leap_year_alt 1900)
let () = assert (is_leap_year 2004 = is_leap_year_alt 2004)
let () = assert (is_leap_year 2001 = is_leap_year_alt 2001)

(* Edge cases *)
let () = assert (not (is_leap_year 1))
let () = assert (not (is_leap_year 3))
let () = assert (is_leap_year 4)
let () = assert (is_leap_year 400)

(* Print results *)
let () = 
  Printf.printf "=== OCaml Leap Year Tests ===\n";
  Printf.printf "Year 2000 (div by 400): %s\n" (if is_leap_year 2000 then "LEAP" else "NOT LEAP");
  Printf.printf "Year 1900 (div by 100): %s\n" (if is_leap_year 1900 then "LEAP" else "NOT LEAP");
  Printf.printf "Year 2004 (div by 4):   %s\n" (if is_leap_year 2004 then "LEAP" else "NOT LEAP");
  Printf.printf "Year 2001 (not div 4):  %s\n" (if is_leap_year 2001 then "LEAP" else "NOT LEAP");
  Printf.printf "\nAll assertions passed!\n"
