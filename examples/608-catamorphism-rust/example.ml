(* Catamorphisms in OCaml *)

(* Nat catamorphism *)
let cata_nat zero succ n =
  let rec go = function
    | 0 -> zero
    | n -> succ (go (n-1))
  in go n

let to_int n = cata_nat 0 (fun x -> x+1) n
let double  n = cata_nat 0 (fun x -> x+2) n

(* List catamorphism = fold_right *)
let cata_list nil cons xs = List.fold_right cons xs nil

let sum   = cata_list 0 (+)
let prod  = cata_list 1 ( * )
let rev   = cata_list [] (fun x acc -> acc @ [x])
let len   = cata_list 0 (fun _ acc -> acc+1)

let () =
  Printf.printf "sum [1..5]  = %d\n" (sum [1;2;3;4;5]);
  Printf.printf "prod [1..5] = %d\n" (prod [1;2;3;4;5]);
  Printf.printf "len [1..5]  = %d\n" (len [1;2;3;4;5])
