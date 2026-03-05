(* Coproduct (sum type) in OCaml *)
type ('a,'b) either = Left of 'a | Right of 'b

(* Injection morphisms *)
let inl : 'a -> ('a,'b) either = fun a -> Left a
let inr : 'b -> ('a,'b) either = fun b -> Right b

(* Elimination: the universal property *)
let either : ('a -> 'c) -> ('b -> 'c) -> ('a,'b) either -> 'c =
  fun f g -> function Left a -> f a | Right b -> g b

(* Bifunctor map *)
let bimap f g = function Left a -> Left (f a) | Right b -> Right (g b)

let () =
  let xs : (int, string) either list = [Left 1; Right "hello"; Left 42; Right "world"] in
  List.iter (fun e ->
    let desc = either (fun n -> Printf.sprintf "int:%d" n)
                      (fun s -> Printf.sprintf "str:%s" s) e in
    Printf.printf "%s\n" desc
  ) xs;
  let doubled = List.map (bimap (fun n->n*2) String.uppercase_ascii) xs in
  List.iter (fun e -> Printf.printf "%s " (either string_of_int Fun.id e)) doubled;
  print_newline ()
