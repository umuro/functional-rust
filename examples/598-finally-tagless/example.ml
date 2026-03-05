(* Finally tagless in OCaml via module functors *)
module type EXPR = sig
  type 'a repr
  val lit : int -> int repr
  val add : int repr -> int repr -> int repr
  val mul : int repr -> int repr -> int repr
end

(* Evaluator *)
module Eval = struct
  type 'a repr = 'a
  let lit n   = n
  let add l r = l + r
  let mul l r = l * r
end

(* Printer *)
module Print = struct
  type 'a repr = string
  let lit n   = string_of_int n
  let add l r = Printf.sprintf "(%s+%s)" l r
  let mul l r = Printf.sprintf "(%s*%s)" l r
end

(* Expression: 3 * 4 + 2 *)
let prog (type a) (module E : EXPR with type 'x repr = a) =
  E.add (E.mul (E.lit 3) (E.lit 4)) (E.lit 2)

let () =
  Printf.printf "eval: %d\n"  (prog (module Eval));
  Printf.printf "print: %s\n" (prog (module Print))
