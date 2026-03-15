(* 473: String Parsing — FromStr / parse()
   Rust's FromStr trait lets types be parsed from strings.
   OCaml idiom: provide an [of_string] function returning
   a result type, and use int_of_string / float_of_string
   (raise exceptions) or their _opt variants for safe parsing. *)

(* A simple RGB color type *)
type color = { r: int; g: int; b: int }

(* Parse "r,g,b" → color; returns Error message on failure *)
let color_of_string s =
  match String.split_on_char ',' s with
  | [rs; gs; bs] ->
    (match int_of_string_opt (String.trim rs),
           int_of_string_opt (String.trim gs),
           int_of_string_opt (String.trim bs) with
     | Some r, Some g, Some b
       when r >= 0 && r <= 255
         && g >= 0 && g <= 255
         && b >= 0 && b <= 255 ->
       Ok { r; g; b }
     | _ -> Error ("invalid color: " ^ s))
  | _ -> Error ("expected r,g,b format: " ^ s)

(* Generic safe int parse — like "42".parse::<i32>() *)
let parse_int s = int_of_string_opt s

(* Parse a list of ints from comma-separated string *)
let parse_int_list s =
  String.split_on_char ',' s
  |> List.map String.trim
  |> List.map int_of_string_opt
  |> List.fold_right (fun x acc ->
       match x, acc with
       | Some n, Ok xs -> Ok (n :: xs)
       | None,   _     -> Error "parse failed"
       | _,      err   -> err)
     (Ok [])

let () =
  (* int parsing *)
  assert (parse_int "42" = Some 42);
  assert (parse_int "abc" = None);
  Printf.printf "parse_int \"42\"=%s \"abc\"=%s\n%!"
    (parse_int "42"  |> Option.map string_of_int |> Option.value ~default:"None")
    (parse_int "abc" |> Option.map string_of_int |> Option.value ~default:"None");

  (* Color parsing *)
  (match color_of_string "10,20,30" with
   | Ok c ->
     assert (c = { r=10; g=20; b=30 });
     Printf.printf "color: r=%d g=%d b=%d\n%!" c.r c.g c.b
   | Error e -> failwith e);

  (match color_of_string "1,2" with
   | Ok _   -> failwith "should fail"
   | Error e -> Printf.printf "bad color: %s\n%!" e);

  (* List parsing *)
  (match parse_int_list "1,2,3,4" with
   | Ok xs ->
     assert (xs = [1;2;3;4]);
     Printf.printf "int list: %s\n%!"
       (xs |> List.map string_of_int |> String.concat ", ")
   | Error e -> failwith e)
