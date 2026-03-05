(* OCaml: no explicit distinction — all are first-class function values *)

let square x = x * x
let cube   x = x * x * x

(* Higher-order *)
let apply f x = f x
let compose f g x = f (g x)

(* "Table" of transforms *)
let math_ops = [
  ("square", square);
  ("cube",   cube);
  ("double", fun x -> x * 2);
  ("negate", fun x -> -x);
]

let () =
  List.iter (fun (name, f) ->
    Printf.printf "%s(5) = %d\n" name (f 5)
  ) math_ops;

  (* Functions as values — can be passed, returned, stored *)
  let pick_op name =
    List.assoc name math_ops
  in
  let op = pick_op "cube" in
  Printf.printf "picked cube(4) = %d\n" (op 4)
