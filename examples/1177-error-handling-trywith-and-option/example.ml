(* Error Handling — try/with and Option *)
(* Exception handling and conversion to Option/Result *)

(* Convert exception-throwing functions to Option *)
let try_with f x = try Some (f x) with _ -> None

let safe_int_of_string = try_with int_of_string
let safe_hd = try_with List.hd
let safe_find k = try_with (List.assoc k)

let () =
  (match safe_int_of_string "42" with
   | Some n -> Printf.printf "Parsed: %d\n" n
   | None -> print_endline "Failed");
  (match safe_find "x" [("x", 1); ("y", 2)] with
   | Some v -> Printf.printf "Found: %d\n" v
   | None -> print_endline "Not found")
