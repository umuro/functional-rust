(* 1033: Top-K Elements with a Priority Queue
   OCaml stdlib has no built-in heap, so we implement a simple min-heap
   using a sorted approach, or use the functional "sort + take" idiom. *)

(* Top-K using sort: simple and idiomatic for moderate inputs *)
let top_k k data =
  let sorted = List.sort (fun a b -> compare b a) data in  (* descending *)
  let rec take n lst =
    if n = 0 then []
    else match lst with
    | [] -> []
    | x :: rest -> x :: take (n - 1) rest
  in
  take k sorted

(* Top-K with a maintained min-heap via a sorted list (bounded size k) *)
(* Insert into sorted list keeping only the k largest *)
let top_k_bounded k data =
  let insert_sorted x lst =
    let rec aux = function
      | [] -> [x]
      | h :: t -> if x >= h then x :: h :: t else h :: aux t
    in
    aux lst
  in
  let heap =
    List.fold_left
      (fun acc x ->
        let h = insert_sorted x acc in
        (* Keep only top k — drop smallest (last element in descending list) *)
        let rec drop_last = function
          | [] -> []
          | [_] -> []
          | h :: t -> h :: drop_last t
        in
        if List.length h > k then drop_last h else h)
      [] data
  in
  heap  (* already in descending order *)

(* Top-K by a key function *)
let top_k_by k key_fn data =
  let sorted = List.sort (fun a b -> compare (key_fn b) (key_fn a)) data in
  let rec take n = function
    | [] -> []
    | _ when n = 0 -> []
    | x :: rest -> x :: take (n - 1) rest
  in
  take k sorted

let () =
  let data = [3; 1; 4; 1; 5; 9; 2; 6; 5; 3] in

  let top3 = top_k 3 data in
  Printf.printf "Top-3: %s\n"
    (String.concat ", " (List.map string_of_int top3));
  assert (top3 = [9; 6; 5]);

  let top1 = top_k 1 data in
  assert (top1 = [9]);

  let top3b = top_k_bounded 3 data in
  Printf.printf "Top-3 (bounded): %s\n"
    (String.concat ", " (List.map string_of_int top3b));
  assert (top3b = [9; 6; 5]);

  (* Top-K strings by length *)
  let words = ["hi"; "hello"; "hey"; "howdy"; "h"] in
  let longest3 = top_k_by 3 String.length words in
  Printf.printf "Longest-3: %s\n" (String.concat ", " longest3);
  assert (List.length longest3 = 3);
  assert (List.hd longest3 = "howdy" || List.hd longest3 = "hello");

  Printf.printf "All assertions passed.\n"
