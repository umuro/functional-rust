(* Example 082: Type Aliases *)
(* type keyword in both languages *)

(* Approach 1: Simple type aliases *)
type user_id = int
type name = string
type age = int
type user = { id : user_id; uname : name; uage : age }

let create_user id n a : user = { id; uname = n; uage = a }

(* Approach 2: Parameterized type aliases *)
type 'a result_with_msg = ('a * string) option
type 'a validator = 'a -> bool
type ('a, 'b) transform = 'a -> 'b

let validate_positive : int validator = fun x -> x > 0
let int_to_string : (int, string) transform = string_of_int

(* Approach 3: Complex type aliases for readability *)
type point = float * float
type line = point * point
type polygon = point list

let distance ((x1, y1) : point) ((x2, y2) : point) : float =
  sqrt ((x2 -. x1) ** 2.0 +. (y2 -. y1) ** 2.0)

let perimeter (poly : polygon) : float =
  match poly with
  | [] | [_] -> 0.0
  | first :: _ ->
    let rec aux acc = function
      | [] -> acc
      | [last] -> acc +. distance last first
      | a :: (b :: _ as rest) -> aux (acc +. distance a b) rest
    in
    aux 0.0 poly

type 'a predicate = 'a -> bool
type 'a comparator = 'a -> 'a -> int

let filter_with (pred : 'a predicate) lst = List.filter pred lst
let sort_with (cmp : 'a comparator) lst = List.sort cmp lst

(* Tests *)
let () =
  let u = create_user 1 "Alice" 30 in
  assert (u.id = 1);
  assert (u.uname = "Alice");

  assert (validate_positive 5 = true);
  assert (validate_positive (-1) = false);
  assert (int_to_string 42 = "42");

  let p1 : point = (0.0, 0.0) in
  let p2 : point = (3.0, 4.0) in
  assert (abs_float (distance p1 p2 -. 5.0) < 0.001);

  let square : polygon = [(0.0, 0.0); (1.0, 0.0); (1.0, 1.0); (0.0, 1.0)] in
  assert (abs_float (perimeter square -. 4.0) < 0.001);

  let evens = filter_with (fun x -> x mod 2 = 0) [1;2;3;4;5;6] in
  assert (evens = [2;4;6]);

  Printf.printf "✓ All tests passed\n"
