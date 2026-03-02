(* List Operations and Recursion in OCaml *)

(* Basic recursive list operations *)
let rec length lst =
  match lst with
  | [] -> 0
  | _ :: tail -> 1 + length tail

let rec sum lst =
  match lst with
  | [] -> 0
  | head :: tail -> head + sum tail

let rec append lst1 lst2 =
  match lst1 with
  | [] -> lst2
  | head :: tail -> head :: append tail lst2

(* Tail-recursive versions for efficiency *)
let length_tr lst =
  let rec aux acc = function
    | [] -> acc
    | _ :: tail -> aux (acc + 1) tail
  in
  aux 0 lst

let sum_tr lst =
  let rec aux acc = function
    | [] -> acc
    | head :: tail -> aux (acc + head) tail
  in
  aux 0 lst

(* Map and filter using recursion *)
let rec map f lst =
  match lst with
  | [] -> []
  | head :: tail -> f head :: map f tail

let rec filter pred lst =
  match lst with
  | [] -> []
  | head :: tail ->
      if pred head then head :: filter pred tail
      else filter pred tail

(* Take and drop *)
let rec take n lst =
  match (n, lst) with
  | (0, _) | (_, []) -> []
  | (n, head :: tail) -> head :: take (n - 1) tail

let rec drop n lst =
  match (n, lst) with
  | (0, _) -> lst
  | (_, []) -> []
  | (n, _ :: tail) -> drop (n - 1) tail

(* Examples *)
let () =
  let numbers = [1; 2; 3; 4; 5] in
  
  Printf.printf "List: [%s]\n"
    (String.concat "; " (List.map string_of_int numbers));
  
  Printf.printf "Length: %d\n" (length numbers);
  Printf.printf "Sum: %d\n" (sum numbers);
  
  let doubled = map (fun x -> x * 2) numbers in
  Printf.printf "Doubled: [%s]\n"
    (String.concat "; " (List.map string_of_int doubled));
  
  let evens = filter (fun x -> x mod 2 = 0) numbers in
  Printf.printf "Evens: [%s]\n"
    (String.concat "; " (List.map string_of_int evens));
  
  Printf.printf "Take 3: [%s]\n"
    (String.concat "; " (List.map string_of_int (take 3 numbers)));
  
  Printf.printf "Drop 2: [%s]\n"
    (String.concat "; " (List.map string_of_int (drop 2 numbers)))
