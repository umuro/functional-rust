(* Algebraic Data Types: Option and Result *)

(* Option type for nullable values *)
let safe_divide x y =
  if y = 0 then None
  else Some (x / y)

let rec find_index pred lst =
  let rec aux i = function
    | [] -> None
    | head :: tail ->
        if pred head then Some i
        else aux (i + 1) tail
  in
  aux 0 lst

(* Result type for error handling *)
type ('a, 'e) result = Ok of 'a | Error of 'e

let parse_int s =
  try Ok (int_of_string s)
  with Failure _ -> Error ("Not a valid integer: " ^ s)

let safe_sqrt x =
  if x < 0.0 then Error "Cannot sqrt negative"
  else Ok (sqrt x)

(* Chaining with bind (monadic operations) *)
let (>>=) opt f =
  match opt with
  | None -> None
  | Some x -> f x

let (>>|) opt f =
  match opt with
  | None -> None
  | Some x -> Some (f x)

(* Result bind *)
let bind_result res f =
  match res with
  | Error e -> Error e
  | Ok x -> f x

(* Combinators *)
let option_map f = function
  | None -> None
  | Some x -> Some (f x)

let option_default default = function
  | None -> default
  | Some x -> x

let result_map f = function
  | Error e -> Error e
  | Ok x -> Ok (f x)

(* Examples *)
let () =
  (* Option examples *)
  (match safe_divide 10 2 with
   | Some n -> Printf.printf "10 / 2 = %d\n" n
   | None -> Printf.printf "Division by zero\n");
  
  (match safe_divide 10 0 with
   | Some n -> Printf.printf "10 / 0 = %d\n" n
   | None -> Printf.printf "Division by zero\n");
  
  let numbers = [1; 3; 5; 7] in
  (match find_index (fun x -> x > 4) numbers with
   | Some i -> Printf.printf "First > 4 at index %d\n" i
   | None -> Printf.printf "Not found\n");
  
  (* Result examples *)
  (match parse_int "42" with
   | Ok n -> Printf.printf "Parsed: %d\n" n
   | Error msg -> Printf.printf "Error: %s\n" msg);
  
  (match parse_int "hello" with
   | Ok n -> Printf.printf "Parsed: %d\n" n
   | Error msg -> Printf.printf "Error: %s\n" msg);
  
  (* Chaining *)
  let result =
    safe_divide 100 5
    >>= (fun x -> safe_divide x 2)
    >>| (fun x -> x * 2)
  in
  Printf.printf "Chained: %s\n"
    (match result with Some n -> string_of_int n | None -> "None")
