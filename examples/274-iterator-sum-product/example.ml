(* 274: sum and product — numeric reductions over a sequence.
   OCaml: List.fold_left with (+) and (*), or direct helpers. *)

let sum lst = List.fold_left ( + ) 0 lst
let product lst = List.fold_left ( * ) 1 lst

let sum_float lst = List.fold_left ( +. ) 0.0 lst

let () =
  (* Gauss sum: 1+2+...+100 = 5050 *)
  let gauss = List.init 100 (fun i -> i + 1) |> sum in
  Printf.printf "sum 1..100 = %d\n" gauss;

  (* Factorial: 5! = 120 *)
  let fact5 = List.init 5 (fun i -> i + 1) |> product in
  Printf.printf "5! = %d\n" fact5;

  (* Empty sum: identity is 0 *)
  Printf.printf "sum [] = %d\n" (sum []);

  (* Empty product: identity is 1 *)
  Printf.printf "product [] = %d\n" (product []);

  (* Float sum *)
  let total = sum_float [1.5; 2.5; 3.0] in
  Printf.printf "sum [1.5;2.5;3.0] = %.1f\n" total;

  (* Sum of squares *)
  let sum_sq = List.init 5 (fun i -> (i+1) * (i+1)) |> sum in
  Printf.printf "sum of squares 1..5 = %d\n" sum_sq;

  (* Lazy Seq sum *)
  let lazy_sum = Seq.ints 1 |> Seq.take 100
                 |> Seq.fold_left ( + ) 0 in
  Printf.printf "seq sum 1..100 = %d\n" lazy_sum;

  (* Product with fold_right preserves left-to-right order for verification *)
  let fact4 = List.fold_right ( * ) [1;2;3;4] 1 in
  Printf.printf "4! (fold_right) = %d\n" fact4
