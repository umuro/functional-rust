(* Function Composition *)

(* Composition operator *)
let (<<) f g x = f (g x)
let (>>) f g x = g (f x)

(* Examples *)
let double x = x * 2
let add3 x = x + 3
let square x = x * x

(* Composed functions *)
let double_then_add3 = double >> add3
let add3_then_double = add3 >> double
let complex = square >> double >> add3

(* Point-free style *)
let process = List.map (square >> double)

(* Examples *)
let () =
  Printf.printf "double_then_add3 5 = %d\n" (double_then_add3 5);
  Printf.printf "add3_then_double 5 = %d\n" (add3_then_double 5);
  Printf.printf "complex 4 = %d\n" (complex 4);
  
  let numbers = [1; 2; 3; 4] in
  let processed = process numbers in
  Printf.printf "Processed: [%s]\n"
    (String.concat "; " (List.map string_of_int processed))
