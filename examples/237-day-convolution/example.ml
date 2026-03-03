(* Day convolution: a way to combine two functors.
   Day f g a = exists b c. f b * g c * (b -> c -> a)
   Used to build applicative functors compositionally. *)

(* Simplified Day convolution for lists *)
(* Day List List a = all ways to combine elements *)
type ('f, 'g, 'a) day =
  | Day : 'f * 'g * ('b -> 'c -> 'a) -> ('f, 'g, 'a) day

(* Day convolution of two lists = cartesian product *)
let day_product xs ys f =
  List.concat_map (fun x ->
    List.map (fun y -> f x y) ys
  ) xs

(* This IS the applicative for lists *)
let apply_list fs xs = day_product fs xs (fun f x -> f x)

let () =
  let xs = [1; 2; 3] in
  let ys = [10; 20] in

  (* Day convolution: all sums *)
  let sums = day_product xs ys ( + ) in
  Printf.printf "all sums: [%s]\n" (sums |> List.map string_of_int |> String.concat ";");

  (* All products *)
  let prods = day_product xs ys ( * ) in
  Printf.printf "all products: [%s]\n" (prods |> List.map string_of_int |> String.concat ";");

  (* Applicative from Day *)
  let fs = [(fun x -> x * 2); (fun x -> x + 1)] in
  let results = apply_list fs [1; 2; 3] in
  Printf.printf "apply: [%s]\n" (results |> List.map string_of_int |> String.concat ";")
