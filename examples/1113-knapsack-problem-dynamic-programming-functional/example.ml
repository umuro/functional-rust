(* OCaml implementation of the 0/1 Knapsack problem with dynamic programming (functional style) *)

(* Knapsack function: using an array for memoization *)
let knapsack (weights : int array) (values : int array) (capacity : int) : int = 
  let n = Array.length weights in
  let dp = Array.make (capacity + 1) 0 in (* dp[w] = max value for capacity w *)

  for i = 0 to n - 1 do
    (* Iterate backwards to use values from current dp row only, not future values *)
    for w = capacity downto 0 do
      if weights.(i) <= w then
        dp.(w) <- max dp.(w) (values.(i) + dp.(w - weights.(i)))
    done
  done;
  dp.(capacity)

(* Test cases *)
let () = 
  let weights1 = [|1; 2; 3|] in
  let values1 = [|10; 15; 40|] in
  let capacity1 = 6 in
  let result1 = knapsack weights1 values1 capacity1 in
  Printf.printf "Knapsack 1: %d (Expected: 65)\n" result1;
  assert (result1 = 65);

  let weights2 = [||] in
  let values2 = [||] in
  let capacity2 = 10 in
  let result2 = knapsack weights2 values2 capacity2 in
  Printf.printf "Knapsack 2: %d (Expected: 0)\n" result2;
  assert (result2 = 0);

  let weights3 = [|10|] in
  let values3 = [|100|] in
  let capacity3 = 5 in
  let result3 = knapsack weights3 values3 capacity3 in
  Printf.printf "Knapsack 3: %d (Expected: 0)\n" result3;
  assert (result3 = 0);

  let weights4 = [|2; 3; 4; 5|] in
  let values4 = [|3; 4; 5; 6|] in
  let capacity4 = 5 in
  let result4 = knapsack weights4 values4 capacity4 in
  Printf.printf "Knapsack 4: %d (Expected: 7)\n" result4;
  assert (result4 = 7);

  print_endline "All Knapsack tests passed!";

  (* The example from the Rust code: *)
  let weights_rust_ex = [|3; 4; 6; 5|] in
  let values_rust_ex = [|2; 3; 1; 4|] in
  let capacity_rust_ex = 8 in
  let result_rust_ex = knapsack weights_rust_ex values_rust_ex capacity_rust_ex in
  Printf.printf "Rust example values: %d (Expected: 6)\n" result_rust_ex;
  assert (result_rust_ex = 6)
