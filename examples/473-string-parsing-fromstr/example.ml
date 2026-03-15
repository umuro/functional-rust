(* 473. FromStr / parse – OCaml *)
type color = { r:int; g:int; b:int }

let parse_color s =
  match String.split_on_char ',' s with
  | [r;g;b] -> (match int_of_string_opt(String.trim r),
                      int_of_string_opt(String.trim g),
                      int_of_string_opt(String.trim b) with
               | Some r,Some g,Some b -> Ok{r;g;b}
               | _ -> Error "bad values")
  | _ -> Error "wrong format"

let () =
  Printf.printf "%d\n" (int_of_string "42");
  Printf.printf "%.2f\n" (float_of_string "3.14");
  (match parse_color "255,128,0" with
   | Ok c -> Printf.printf "r=%d g=%d b=%d\n" c.r c.g c.b
   | Error e -> print_string e);
  Printf.printf "bad=%b\n" (parse_color "x,y,z" = Error "bad values")
