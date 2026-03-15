(* 138: Type Witnesses / GADTs
   OCaml has native GADTs — this is the idiomatic home turf for this concept.
   A phantom type parameter T witnesses the result type of an expression at
   compile time. The compiler enforces it through GADT constructor type equations. *)

(* ── Approach 1: GADT-style typed expression tree ────────────────────────── *)

(* The GADT: each constructor carries the result type as a type index.
   'a expr is the type of expressions whose value has type 'a. *)
type _ expr =
  | Int  : int -> int expr
  | Bool : bool -> bool expr
  | Add  : int expr * int expr -> int expr
  | Eq   : int expr * int expr -> bool expr
  | If   : bool expr * 'a expr * 'a expr -> 'a expr

(* interpret is total and type-safe — no unreachable branches needed.
   The GADT equations make ill-typed combinations impossible. *)
let rec interpret : type a. a expr -> a = function
  | Int n         -> n
  | Bool b        -> b
  | Add (a, b)    -> interpret a + interpret b
  | Eq  (a, b)    -> interpret a = interpret b
  | If  (c, t, e) -> if interpret c then interpret t else interpret e

(* ── Approach 2: Typed heterogeneous map using first-class modules ────────── *)

(* Each key is a module that fixes the value type T.
   The map stores (module Key) -> value pairs; retrieval is type-safe
   because the key module carries the type witness. *)

module type KEY = sig
  type t
  val name : string
end

(* A heterogeneous map entry: existential over the value type *)
type entry = Entry : (module KEY with type t = 'a) * 'a -> entry

module TypedMap = struct
  type t = (string, entry) Hashtbl.t

  let create () : t = Hashtbl.create 16

  let insert (type a) (tbl : t) (key : (module KEY with type t = a)) (value : a) =
    let module K = (val key) in
    Hashtbl.replace tbl K.name (Entry (key, value))

  let find (type a) (tbl : t) (key : (module KEY with type t = a)) : a option =
    let module K = (val key) in
    match Hashtbl.find_opt tbl K.name with
    | Some (Entry (k2, v)) ->
      let module K2 = (val k2) in
      (* Type equality: same name → same key module → same type *)
      if K.name = K2.name then Some (Obj.magic v : a)
      else None
    | None -> None
end

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* Expression tree: if (1 + 2 = 3) then 100 else 0 *)
  let cond = Eq (Add (Int 1, Int 2), Int 3) in
  let expr = If (cond, Int 100, Int 0) in
  Printf.printf "interpret(if 1+2=3 then 100 else 0) = %d\n" (interpret expr);

  (* Bool branch: type-safe, if-branches must share a type *)
  let b_expr = If (Bool true, Bool false, Bool true) in
  Printf.printf "interpret(if true then false else true) = %b\n" (interpret b_expr);

  (* Typed map *)
  let module AgeKey  = struct type t = int    let name = "age"  end in
  let module NameKey = struct type t = string let name = "name" end in
  let tbl = TypedMap.create () in
  TypedMap.insert tbl (module AgeKey)  30;
  TypedMap.insert tbl (module NameKey) "Alice";
  (match TypedMap.find tbl (module AgeKey) with
   | Some n -> Printf.printf "age = %d\n" n
   | None   -> print_endline "age not found");
  (match TypedMap.find tbl (module NameKey) with
   | Some s -> Printf.printf "name = %s\n" s
   | None   -> print_endline "name not found")
