(* 082: Type Aliases *)

(* Approach 1: Simple aliases *)
type point = float * float
type name = string
type age = int

let distance ((x1, y1) : point) ((x2, y2) : point) : float =
  sqrt ((x2 -. x1) ** 2.0 +. (y2 -. y1) ** 2.0)

(* Approach 2: Result alias *)
type error = string
type 'a result_t = ('a, error) result

let parse_int (s : string) : int result_t =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "Not a number: %s" s)

let safe_div (a : int) (b : int) : int result_t =
  if b = 0 then Error "Division by zero" else Ok (a / b)

(* Approach 3: Complex type alias *)
type 'a predicate = 'a -> bool
type 'a transform = 'a -> 'a
type ('a, 'b) mapper = 'a -> 'b

let filter_map (pred : 'a predicate) (f : ('a, 'b) mapper) lst =
  lst |> List.filter pred |> List.map f

(* Tests *)
let () =
  let p1 : point = (0.0, 0.0) in
  let p2 : point = (3.0, 4.0) in
  assert (abs_float (distance p1 p2 -. 5.0) < 0.001);
  assert (parse_int "42" = Ok 42);
  assert (parse_int "abc" = Error "Not a number: abc");
  assert (safe_div 10 3 = Ok 3);
  let is_even : int predicate = fun x -> x mod 2 = 0 in
  let double : (int, int) mapper = fun x -> x * 2 in
  assert (filter_map is_even double [1; 2; 3; 4; 5; 6] = [4; 8; 12]);
  Printf.printf "✓ All tests passed\n"
