let numbers = [1; 2; 3; 4; 5]
let sum = List.fold_left ( + ) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let max_val = List.fold_left max min_int numbers
let () = Printf.printf "Sum: %d, Product: %d, Max: %d\n" sum product max_val