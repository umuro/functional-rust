(* 494. Number <-> String – OCaml *)
let () =
  Printf.printf "%s\n" (string_of_int 42);
  Printf.printf "%s\n" (string_of_float 3.14);
  Printf.printf "%d\n" (int_of_string "42");
  Printf.printf "%.2f\n" (float_of_string "3.14");
  (match int_of_string_opt "abc" with Some _ -> () | None -> print_string "not a number\n");
  Printf.printf "hex: %x\n" 255;
  Printf.printf "from hex: %d\n" (int_of_string "0xff")
