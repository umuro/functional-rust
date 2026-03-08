(* Recursive quicksort with higher-order comparator *)
let rec quicksort gt = function
  | [] -> []
  | x::xs ->
      let ys, zs = List.partition (gt x) xs in
      (quicksort gt ys) @ (x :: (quicksort gt zs))

(* Test suite *)
let () =
  (* Test 1: Empty list *)
  assert ((quicksort (>) []) = []);
  Printf.printf "✓ Test 1: Empty list\n";

  (* Test 2: Single element *)
  assert ((quicksort (>) [42]) = [42]);
  Printf.printf "✓ Test 2: Single element\n";

  (* Test 3: Multiple elements (ascending) *)
  let input = [4; 65; 2; -31; 0; 99; 83; 782; 1] in
  let expected = [-31; 0; 1; 2; 4; 65; 83; 99; 782] in
  assert ((quicksort (>) input) = expected);
  Printf.printf "✓ Test 3: Multiple elements (ascending)\n";

  (* Test 4: Already sorted *)
  assert ((quicksort (>) [1; 2; 3; 4; 5]) = [1; 2; 3; 4; 5]);
  Printf.printf "✓ Test 4: Already sorted\n";

  (* Test 5: Reverse sorted *)
  assert ((quicksort (>) [5; 4; 3; 2; 1]) = [1; 2; 3; 4; 5]);
  Printf.printf "✓ Test 5: Reverse sorted\n";

  (* Test 6: Duplicates *)
  assert ((quicksort (>) [3; 1; 3; 2; 1; 3]) = [1; 1; 2; 3; 3; 3]);
  Printf.printf "✓ Test 6: Duplicates\n";

  (* Test 7: Custom comparator (descending) *)
  let expected_desc = [782; 99; 83; 65; 4; 2; 1; 0; -31] in
  assert ((quicksort (<) [4; 65; 2; -31; 0; 99; 83; 782; 1]) = expected_desc);
  Printf.printf "✓ Test 7: Custom comparator (descending)\n";

  (* Test 8: Negative numbers *)
  assert ((quicksort (>) [-5; -1; -10; 0; 5; 1]) = [-10; -5; -1; 0; 1; 5]);
  Printf.printf "✓ Test 8: Negative numbers\n";

  Printf.printf "\n✓ All tests passed!\n"
