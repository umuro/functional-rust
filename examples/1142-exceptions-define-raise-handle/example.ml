(* Exceptions — Define, Raise, Handle *)
(* Custom exceptions and exception handling *)

exception Invalid_input of string
exception Out_of_range of { value: int; min: int; max: int }

let safe_sqrt x =
  if x < 0.0 then raise (Invalid_input "negative number")
  else sqrt x

let clamp ~min ~max x =
  if x < min || x > max then
    raise (Out_of_range { value = x; min; max })
  else x

let () =
  (try Printf.printf "sqrt(4) = %.1f\n" (safe_sqrt 4.0) with _ -> ());
  (try ignore (safe_sqrt (-1.0)) with
   | Invalid_input msg -> Printf.printf "Error: %s\n" msg);
  (try ignore (clamp ~min:0 ~max:100 150) with
   | Out_of_range r -> Printf.printf "Out of range: %d not in [%d,%d]\n" r.value r.min r.max)
