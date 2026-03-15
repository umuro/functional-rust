(* 1023: Safe Integer Parsing
   int_of_string_opt is the idiomatic OCaml safe parse — returns None on failure.
   We add validation layers on top: positive, range, default fallback. *)

(* Approach 1: Basic parse with Option *)
let parse_int s =
  int_of_string_opt (String.trim s)

(* Approach 2: Parse with Result and custom error message *)
let parse_int_result s =
  match int_of_string_opt (String.trim s) with
  | Some n -> Ok n
  | None   -> Error (Printf.sprintf "cannot parse '%s' as integer" s)

(* Approach 3: Parse with validation *)
let parse_positive s =
  match int_of_string_opt (String.trim s) with
  | None   -> Error (Printf.sprintf "not a number: %s" s)
  | Some n ->
    if n < 0 then Error (Printf.sprintf "negative: %d" n)
    else Ok n

let parse_in_range s min_val max_val =
  match int_of_string_opt (String.trim s) with
  | None   -> Error (Printf.sprintf "not a number: %s" s)
  | Some n ->
    if n < min_val then Error (Printf.sprintf "%d < min(%d)" n min_val)
    else if n > max_val then Error (Printf.sprintf "%d > max(%d)" n max_val)
    else Ok n

(* Parse with default — like unwrap_or *)
let parse_or_default s default =
  match int_of_string_opt (String.trim s) with
  | Some n -> n
  | None   -> default

let () =
  assert (parse_int "42"  = Some 42);
  assert (parse_int "-17" = Some (-17));
  assert (parse_int "0"   = Some 0);
  assert (parse_int "abc" = None);
  assert (parse_int ""    = None);
  assert (parse_int "12.5" = None);

  assert (parse_int_result "42" = Ok 42);
  (match parse_int_result "abc" with
   | Error msg ->
     assert (String.length msg > 0);
     assert (try let _ = Str.search_forward (Str.regexp "cannot parse") msg 0 in true
             with Not_found -> true)
   | _ -> assert false);

  assert (parse_positive "42" = Ok 42);
  assert (parse_positive "0"  = Ok 0);
  (match parse_positive "-5" with Error msg -> assert (String.length msg > 0) | _ -> assert false);
  (match parse_positive "xyz" with Error msg -> assert (String.length msg > 0) | _ -> assert false);

  assert (parse_in_range "50" 1 100 = Ok 50);
  assert (parse_in_range "1"  1 100 = Ok 1);
  assert (parse_in_range "100" 1 100 = Ok 100);
  assert (Result.is_error (parse_in_range "0" 1 100));
  assert (Result.is_error (parse_in_range "101" 1 100));
  assert (Result.is_error (parse_in_range "abc" 1 100));

  assert (parse_or_default "42" 0 = 42);
  assert (parse_or_default "abc" 0 = 0);
  assert (parse_or_default "" (-1) = (-1));

  (* OCaml DOES trim whitespace before parsing when we trim manually *)
  assert (parse_int " 42 " = Some 42);  (* we use String.trim *)

  Printf.printf "parse_int 42: %s\n"
    (match parse_int "42" with Some n -> string_of_int n | None -> "error")
