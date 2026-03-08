(* OCaml: List Filtering Example *)
(* Demonstrates idiomatic OCaml approaches to filtering lists *)

(* Idiomatic OCaml: using List.filter *)
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds = List.filter (fun x -> x mod 2 <> 0) numbers

(* Recursive functional style *)
let rec filter_recursive pred lst =
  match lst with
  | [] -> []
  | head :: tail ->
      if pred head then
        head :: filter_recursive pred tail
      else
        filter_recursive pred tail

let evens_recursive = filter_recursive (fun x -> x mod 2 = 0) numbers
let odds_recursive = filter_recursive (fun x -> x mod 2 <> 0) numbers

(* Helper function to format lists *)
let format_list lst =
  String.concat ", " (List.map string_of_int lst)

(* Output *)
let () = Printf.printf "Evens: %s\n" (format_list evens)
let () = Printf.printf "Odds: %s\n" (format_list odds)
let () = Printf.printf "Evens (recursive): %s\n" (format_list evens_recursive)
let () = Printf.printf "Odds (recursive): %s\n" (format_list odds_recursive)

(* Unit tests using assert *)
let () =
  (* Test filter_recursive with evens *)
  assert (filter_recursive (fun x -> x mod 2 = 0) [1; 2; 3; 4; 5; 6] = [2; 4; 6]);
  
  (* Test filter_recursive with odds *)
  assert (filter_recursive (fun x -> x mod 2 <> 0) [1; 2; 3; 4; 5; 6] = [1; 3; 5]);
  
  (* Test empty list *)
  assert (filter_recursive (fun x -> x mod 2 = 0) [] = []);
  
  (* Test single element match *)
  assert (filter_recursive (fun x -> x mod 2 = 0) [4] = [4]);
  
  (* Test single element no match *)
  assert (filter_recursive (fun x -> x mod 2 = 0) [3] = []);
  
  (* Test all match *)
  assert (filter_recursive (fun x -> x mod 2 = 0) [2; 4; 6; 8] = [2; 4; 6; 8]);
  
  (* Test none match *)
  assert (filter_recursive (fun x -> x mod 2 = 0) [1; 3; 5; 7] = []);
  
  (* Test complex predicate *)
  let between_3_and_7 = filter_recursive (fun x -> x > 3 && x < 8) [1; 2; 3; 4; 5; 6; 7; 8] in
  assert (between_3_and_7 = [4; 5; 6; 7]);
  
  (* Verify List.filter and filter_recursive give same results *)
  let test_list = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in
  let pred = fun x -> x mod 2 = 0 in
  assert (List.filter pred test_list = filter_recursive pred test_list);
  
  Printf.printf "All tests passed!\n"
