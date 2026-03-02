(* Currying and Partial Application *)

(* Curried functions (default in OCaml) *)
let add x y = x + y
let multiply x y z = x * y * z

(* Partial application *)
let add5 = add 5
let double = multiply 2
let triple = multiply 3

(* Higher-order functions with currying *)
let map f lst =
  List.map f lst

let filter pred lst =
  List.filter pred lst

(* Custom operators *)
let (|>) x f = f x
let (@@) f x = f x

(* Examples *)
let () =
  Printf.printf "add5 10 = %d\n" (add5 10);
  Printf.printf "double 3 4 = %d\n" (double 3 4);
  
  let numbers = [1; 2; 3; 4; 5] in
  let add10 = add 10 in
  let plus10 = map add10 numbers in
  Printf.printf "Add 10: [%s]\n"
    (String.concat "; " (List.map string_of_int plus10));
  
  let gt3 = fun x -> x > 3 in
  let filtered = filter gt3 numbers in
  Printf.printf "Greater than 3: [%s]\n"
    (String.concat "; " (List.map string_of_int filtered));
  
  (* Pipeline operator *)
  let result = 5 |> add 3 |> multiply 2 1 in
  Printf.printf "Pipeline: %d\n" result
