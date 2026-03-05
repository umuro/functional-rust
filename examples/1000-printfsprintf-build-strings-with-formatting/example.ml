(* Printf.sprintf — Build Strings with Formatting *)
(* Create formatted strings without printing *)

let format_record name age score =
  Printf.sprintf "%-15s | %3d | %6.2f" name age score

let header = Printf.sprintf "%-15s | %3s | %6s" "Name" "Age" "Score"
let sep = String.make (String.length header) '-'

let () =
  print_endline header;
  print_endline sep;
  print_endline (format_record "Alice" 30 95.5);
  print_endline (format_record "Bob" 25 87.3);
  print_endline (format_record "Carol" 28 92.1)
