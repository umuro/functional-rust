(* Example 128: Type-Level Booleans in OCaml *)

(* Approach 1: Phantom type booleans using type aliases *)
type true_t = True_t
type false_t = False_t

(* 'b is a phantom parameter — it's never used in the record body *)
type 'b flag = { _phantom : unit }

let mk_true  : true_t  flag = { _phantom = () }
let mk_false : false_t flag = { _phantom = () }

(* Extract the phantom type information at runtime via a module *)
module type BOOL = sig
  type t
  val value : bool
end

module TrueM : BOOL = struct
  type t = true_t
  let value = true
end

module FalseM : BOOL = struct
  type t = false_t
  let value = false
end

(* Approach 2: GADT-based type-level bool for type-safe branching *)
type _ tbool =
  | TTrue  : true_t  tbool
  | TFalse : false_t tbool

let tbool_value : type a. a tbool -> bool = function
  | TTrue  -> true
  | TFalse -> false

(* Approach 3: Builder pattern — track state in phantom parameters *)
(* OCaml uses abstract types and module signatures to hide constructors *)
module Config : sig
  type ('v, 'l) t
  val create        : string -> int -> (false_t, false_t) t
  val validate      : ('v, 'l) t -> (true_t, 'l) t
  val enable_logging: ('v, 'l) t -> ('v, true_t) t
  val execute       : (true_t, true_t) t -> string
end = struct
  type ('v, 'l) t = { host : string; port : int }
  let create host port = { host; port }
  let validate      cfg = { cfg with host = cfg.host }
  let enable_logging cfg = { cfg with host = cfg.host }
  let execute cfg = Printf.sprintf "Executing on %s:%d" cfg.host cfg.port
end

let () =
  assert (TrueM.value  = true);
  assert (FalseM.value = false);
  assert (tbool_value TTrue  = true);
  assert (tbool_value TFalse = false);
  let result =
    Config.(create "localhost" 9000 |> validate |> enable_logging |> execute)
  in
  assert (result = "Executing on localhost:9000");
  print_endline "ok"
