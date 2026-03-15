(* Example 138: Type Witnesses / GADT Encoding *)

(* Approach 1: GADT expression tree with type safety *)
type _ expr =
  | IntLit : int -> int expr
  | BoolLit : bool -> bool expr
  | Add : int expr * int expr -> int expr
  | Eq : int expr * int expr -> bool expr
  | If : bool expr * 'a expr * 'a expr -> 'a expr
  | Pair : 'a expr * 'b expr -> ('a * 'b) expr
  | Fst : ('a * 'b) expr -> 'a expr

let rec eval : type a. a expr -> a = function
  | IntLit n -> n
  | BoolLit b -> b
  | Add (a, b) -> eval a + eval b
  | Eq (a, b) -> eval a = eval b
  | If (cond, t, f) -> if eval cond then eval t else eval f
  | Pair (a, b) -> (eval a, eval b)
  | Fst p -> fst (eval p)

(* Approach 2: Type witness for safe casting *)
type (_, _) eq = Refl : ('a, 'a) eq

let cast : type a b. (a, b) eq -> a -> b = fun Refl x -> x

(* Approach 3: Typed keys for heterogeneous map *)
type _ key =
  | IntKey : string -> int key
  | StringKey : string -> string key
  | BoolKey : string -> bool key

type binding = Binding : 'a key * 'a -> binding

let get_int (IntKey _ as k) bindings =
  List.find_map (fun (Binding (k', v)) ->
    match k, k' with
    | IntKey a, IntKey b when a = b -> Some v
    | _ -> None
  ) bindings

(* Tests *)
let () =
  let e = If (Eq (IntLit 1, IntLit 1), IntLit 42, IntLit 0) in
  assert (eval e = 42);

  let e2 = Add (IntLit 10, IntLit 32) in
  assert (eval e2 = 42);

  let e3 = Fst (Pair (IntLit 1, BoolLit true)) in
  assert (eval e3 = 1);

  let x = cast Refl 42 in
  assert (x = 42);

  Printf.printf "✓ All tests passed\n"
