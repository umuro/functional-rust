(* Currying and Partial Application *)
(* Curried functions and partial application patterns *)

let add x y = x + y
let add5 = add 5
let () = Printf.printf "add5 3 = %d\n" (add5 3)

let multiply x y = x * y
let double = multiply 2
let triple = multiply 3

let clamp ~min ~max x =
  if x < min then min else if x > max then max else x

let clamp_percent = clamp ~min:0 ~max:100

let results = List.map clamp_percent [-5; 42; 150; 99]
let () = List.iter (fun x -> Printf.printf "%d " x) results
