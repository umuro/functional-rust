(* Printf and Format — Formatted Output *)
(* Type-safe formatted printing *)

let () =
  Printf.printf "Integer: %d\n" 42;
  Printf.printf "Float: %.2f\n" 3.14159;
  Printf.printf "String: %s\n" "hello";
  Printf.printf "Char: %c\n" 'A';
  Printf.printf "Bool: %b\n" true;
  Printf.printf "Hex: 0x%x, Oct: 0o%o\n" 255 255;
  Printf.printf "Padded: [%10d] [%-10d]\n" 42 42;
  Printf.printf "Zero-padded: [%06d]\n" 42

let msg = Printf.sprintf "(%d, %d)" 10 20
let () = print_endline msg
