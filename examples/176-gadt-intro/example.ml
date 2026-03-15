(* Example 176: Introduction to GADTs *)
(* GADTs (Generalized Algebraic Data Types) allow constructors to return
   different type instantiations of the same type *)

(* Approach 1: Basic GADT with type-safe expressions *)
type _ expr =
  | Int  : int  -> int expr
  | Bool : bool -> bool expr
  | Add  : int expr * int expr -> int expr
  | If   : bool expr * 'a expr * 'a expr -> 'a expr

let rec eval : type a. a expr -> a = function
  | Int n -> n
  | Bool b -> b
  | Add (a, b) -> eval a + eval b
  | If (cond, t, f) -> if eval cond then eval t else eval f

(* Approach 2: GADT for type-safe formatting *)
type _ fmt =
  | Lit  : string -> string fmt
  | FInt : int fmt
  | FStr : string fmt
  | Cat  : 'a fmt * 'b fmt -> ('a * 'b) fmt

let rec format_to_string : type a. a fmt -> a -> string = fun fmt v ->
  match fmt with
  | Lit s -> s
  | FInt -> string_of_int v
  | FStr -> v
  | Cat (a, b) ->
    let (va, vb) = v in
    format_to_string a va ^ format_to_string b vb

(* Approach 3: GADT for type-safe heterogeneous lists *)
type _ hlist =
  | HNil  : unit hlist
  | HCons : 'a * 'b hlist -> ('a * 'b) hlist

let example_list = HCons (42, HCons ("hello", HCons (true, HNil)))

let hd : type a b. (a * b) hlist -> a = function
  | HCons (x, _) -> x

(* Tests *)
let () =
  (* Test Approach 1 *)
  assert (eval (Int 42) = 42);
  assert (eval (Bool true) = true);
  assert (eval (Add (Int 1, Int 2)) = 3);
  assert (eval (If (Bool true, Int 10, Int 20)) = 10);
  assert (eval (If (Bool false, Int 10, Int 20)) = 20);

  (* Test Approach 2 *)
  assert (format_to_string (Lit "hello") "hello" = "hello");
  assert (format_to_string FInt 42 = "42");
  assert (format_to_string FStr "world" = "world");
  assert (format_to_string (Cat (Lit "n=", FInt)) ("n=", 42) = "n=42");

  (* Test Approach 3 *)
  assert (hd example_list = 42);

  print_endline "✓ All tests passed"
