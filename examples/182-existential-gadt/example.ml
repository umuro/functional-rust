(* Example 182: Existential Types *)
(* Hide the concrete type while retaining ability to use it *)

(* Approach 1: Existential via GADT *)
type showable = Show : 'a * ('a -> string) -> showable

let show (Show (x, f)) = f x

let show_list : showable list = [
  Show (42, string_of_int);
  Show ("hello", Fun.id);
  Show (3.14, string_of_float);
  Show (true, string_of_bool);
]

(* Approach 2: First-class module existential *)
module type PRINTABLE = sig
  type t
  val value : t
  val to_string : t -> string
end

let make_printable (type a) (to_s : a -> string) (v : a) : (module PRINTABLE) =
  (module struct
    type t = a
    let value = v
    let to_string = to_s
  end)

let print_it (m : (module PRINTABLE)) =
  let module M = (val m) in
  M.to_string M.value

(* Approach 3: Existential with comparison *)
type comparable = Cmp : 'a * 'a * ('a -> 'a -> int) -> comparable

let compare_pair (Cmp (a, b, cmp)) =
  let r = cmp a b in
  if r < 0 then "less" else if r = 0 then "equal" else "greater"

let () =
  (* Test Approach 1 *)
  let results = List.map show show_list in
  assert (List.nth results 0 = "42");
  assert (List.nth results 1 = "hello");
  assert (List.nth results 3 = "true");

  (* Test Approach 2 *)
  let items = [
    make_printable string_of_int 42;
    make_printable Fun.id "world";
  ] in
  assert (print_it (List.nth items 0) = "42");
  assert (print_it (List.nth items 1) = "world");

  (* Test Approach 3 *)
  assert (compare_pair (Cmp (1, 2, compare)) = "less");
  assert (compare_pair (Cmp (5, 5, compare)) = "equal");
  assert (compare_pair (Cmp ("z", "a", String.compare)) = "greater");

  print_endline "✓ All tests passed"
