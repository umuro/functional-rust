(* Higher-Order Functions in OCaml *)
(* Example: Implementing map, filter, and fold from scratch *)

(* Map: Apply a function to each element *)
let rec map f = function
  | [] -> []
  | x :: xs -> f x :: map f xs

(* Filter: Keep only elements that satisfy a predicate *)
let rec filter pred = function
  | [] -> []
  | x :: xs ->
      if pred x then x :: filter pred xs
      else filter pred xs

(* Fold (reduce): Accumulate a value by applying a function *)
let rec fold_left f acc = function
  | [] -> acc
  | x :: xs -> fold_left f (f acc x) xs

(* Examples *)
let () =
  (* Map: Double each number *)
  let doubled = map (fun x -> x * 2) [1; 2; 3; 4; 5] in
  Printf.printf "Doubled: ";
  List.iter (Printf.printf "%d ") doubled;
  Printf.printf "\n";

  (* Filter: Keep only even numbers *)
  let evens = filter (fun x -> x mod 2 = 0) [1; 2; 3; 4; 5; 6] in
  Printf.printf "Evens: ";
  List.iter (Printf.printf "%d ") evens;
  Printf.printf "\n";

  (* Fold: Sum all numbers *)
  let sum = fold_left (fun acc x -> acc + x) 0 [1; 2; 3; 4; 5] in
  Printf.printf "Sum: %d\n" sum;

  (* Composition: Double, then keep evens, then sum *)
  let result =
    [1; 2; 3; 4; 5]
    |> map (fun x -> x * 2)
    |> filter (fun x -> x mod 2 = 0)
    |> fold_left (fun acc x -> acc + x) 0
  in
  Printf.printf "Composed: %d\n" result
