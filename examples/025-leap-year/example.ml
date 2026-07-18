let leap_year year =
  (year mod 400 = 0) ||
    (year mod 4 = 0 && year mod 100 <> 0)

(* Tests *)
let () =
  assert (leap_year 2000 = true);
  assert (leap_year 1600 = true);
  assert (leap_year 1900 = false);
  assert (leap_year 1800 = false);
  assert (leap_year 2004 = true);
  assert (leap_year 2024 = true);
  assert (leap_year 2001 = false);
  assert (leap_year 2003 = false);
  print_endline "✓ OCaml tests passed"