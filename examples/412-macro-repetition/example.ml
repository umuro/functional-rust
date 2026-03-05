(* Repetition patterns in OCaml — variadic via lists *)

(* Simulate variadic with list arguments *)
let sum_list xs = List.fold_left (+) 0 xs
let product_list xs = List.fold_left ( * ) 1 xs
let all_gt n xs = List.for_all (fun x -> x > n) xs

(* Variadic-like tuple construction *)
let zip3 a b c = List.map2 (fun (x,y) z -> (x,y,z)) (List.combine a b) c

let () =
  Printf.printf "sum [1;2;3;4;5] = %d\n" (sum_list [1;2;3;4;5]);
  Printf.printf "product [1;2;3;4] = %d\n" (product_list [1;2;3;4]);
  Printf.printf "all > 0: %b\n" (all_gt 0 [1;2;3])
