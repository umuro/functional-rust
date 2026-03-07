let leap_year year =
  (year mod 400 = 0) ||
    (year mod 4 = 0 && year mod 100 <> 0)

let () =
  Printf.printf "2000 is leap: %b\n" (leap_year 2000);
  Printf.printf "1900 is leap: %b\n" (leap_year 1900);
  Printf.printf "2004 is leap: %b\n" (leap_year 2004);
  Printf.printf "2001 is leap: %b\n" (leap_year 2001)
