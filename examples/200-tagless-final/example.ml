(* Tagless final: instead of building an AST (initial encoding),
   interpret expressions directly through type class (module type) methods.
   Multiple interpreters without touching the program. *)

module type EXPR = sig
  type 'a repr
  val int  : int  -> int repr
  val bool : bool -> bool repr
  val add  : int repr -> int repr -> int repr
  val mul  : int repr -> int repr -> int repr
  val leq  : int repr -> int repr -> bool repr
  val if_  : bool repr -> 'a repr -> 'a repr -> 'a repr
end

(* Interpreter 1: evaluate *)
module Eval : EXPR = struct
  type 'a repr = 'a
  let int  n     = n
  let bool b     = b
  let add  a b   = a + b
  let mul  a b   = a * b
  let leq  a b   = a <= b
  let if_ c t e  = if c then t else e
end

(* Interpreter 2: pretty-print *)
module Pretty : EXPR = struct
  type 'a repr = string
  let int  n     = string_of_int n
  let bool b     = string_of_bool b
  let add  a b   = Printf.sprintf "(%s + %s)" a b
  let mul  a b   = Printf.sprintf "(%s * %s)" a b
  let leq  a b   = Printf.sprintf "(%s <= %s)" a b
  let if_ c t e  = Printf.sprintf "(if %s then %s else %s)" c t e
end

(* The program — written once, interpreted many ways *)
let program (type a) (module E : EXPR with type 'x repr = 'x a) =
  let open E in
  if_ (leq (add (int 3) (int 4)) (mul (int 2) (int 5)))
      (int 42)
      (int 0)

let () =
  let result = program (module Eval : EXPR with type 'x repr = 'x) in
  Printf.printf "eval:   %d\n" result;
  let printed = program (module Pretty : EXPR with type 'x repr = string) in
  Printf.printf "pretty: %s\n" printed
