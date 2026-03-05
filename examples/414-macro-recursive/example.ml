(* Recursive macros in OCaml — demonstrated via recursive functions *)

(* Stack-based reverse using recursion *)
let rec reverse_acc acc = function
  | [] -> acc
  | x :: xs -> reverse_acc (x :: acc) xs

let reverse xs = reverse_acc [] xs

(* Recursive list processor *)
let rec process_ops acc = function
  | [] -> acc
  | `Add n :: rest -> process_ops (acc + n) rest
  | `Sub n :: rest -> process_ops (acc - n) rest
  | `Mul n :: rest -> process_ops (acc * n) rest

let () =
  Printf.printf "reverse [1;2;3;4;5] = [%s]\n"
    (String.concat ";" (List.map string_of_int (reverse [1;2;3;4;5])));
  let result = process_ops 0 [`Add 10; `Mul 3; `Sub 5] in
  Printf.printf "ops result = %d\n" result
