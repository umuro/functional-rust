(* 082: Type Aliases
   Shorten complex types and encode domain intent *)

(* --- Approach 1: Simple aliases --- *)

type point = float * float     (* alias for a pair *)
type name  = string

let distance (x1, y1) (x2, y2) =
  sqrt ((x2 -. x1) ** 2.0 +. (y2 -. y1) ** 2.0)

(* --- Approach 2: Result alias — common in real codebases --- *)

type app_error =
  | ParseError of string
  | DivByZero

type 'a app_result = ('a, app_error) result   (* parametric alias *)

let parse_int s : int app_result =
  match int_of_string_opt s with
  | None   -> Error (ParseError (Printf.sprintf "Not a number: %s" s))
  | Some n -> Ok n

let safe_div a b : int app_result =
  if b = 0 then Error DivByZero else Ok (a / b)

(* --- Approach 3: Function-type aliases (predicates, transforms) --- *)

type 'a predicate  = 'a -> bool
type 'a transform  = 'a -> 'a

let filter_transform
    (pred : 'a predicate)
    (f    : 'a transform)
    (xs   : 'a list) =
  xs |> List.filter pred |> List.map f

(* IO-like result alias *)
type 'a io_result = ('a, exn) result

let () =
  Printf.printf "distance (0,0) (3,4) = %.1f\n" (distance (0.0, 0.0) (3.0, 4.0));
  Printf.printf "parse_int \"42\"  = %s\n"
    (match parse_int "42" with Ok v -> string_of_int v | Error _ -> "Error");
  Printf.printf "parse_int \"abc\" = %s\n"
    (match parse_int "abc" with Ok _ -> "Ok" | Error (ParseError m) -> "ParseError: " ^ m | Error _ -> "Error");
  Printf.printf "safe_div 10 0 = %s\n"
    (match safe_div 10 0 with Ok v -> string_of_int v | Error DivByZero -> "DivByZero" | Error _ -> "Error");
  let evens_doubled = filter_transform (fun x -> x mod 2 = 0) (fun x -> x * 2) [1;2;3;4;5;6] in
  Printf.printf "even+doubled [1..6] = [%s]\n"
    (String.concat "; " (List.map string_of_int evens_doubled))
