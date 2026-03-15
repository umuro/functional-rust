(* 070: Scan Left — running accumulation *)

(* Approach 1: Recursive scan_left *)
let scan_left f init lst =
  let rec aux acc current = function
    | [] -> List.rev (current :: acc)
    | x :: xs -> aux (current :: acc) (f current x) xs
  in
  aux [] init lst

(* Running sum *)
let running_sum lst = scan_left ( + ) 0 lst

(* Running product *)
let running_product lst = scan_left ( * ) 1 lst

(* Approach 2: Using fold_left to build scan *)
let scan_left_fold f init lst =
  let result, _ =
    List.fold_left
      (fun (acc_list, current) x ->
        let next = f current x in
        (next :: acc_list, next))
      ([init], init)
      lst
  in
  List.rev result

(* Approach 3: Running max *)
let running_max lst =
  match lst with
  | [] -> []
  | x :: xs ->
    scan_left (fun a b -> max a b) x xs

(* Tests *)
let () =
  assert (running_sum [1; 2; 3; 4] = [0; 1; 3; 6; 10]);
  assert (running_sum [] = [0]);
  assert (running_product [1; 2; 3; 4] = [1; 1; 2; 6; 24]);
  assert (scan_left_fold ( + ) 0 [1; 2; 3] = [0; 1; 3; 6]);
  assert (running_max [3; 1; 4; 1; 5; 9] = [3; 3; 4; 4; 5; 9]);
  Printf.printf "✓ All tests passed\n"
