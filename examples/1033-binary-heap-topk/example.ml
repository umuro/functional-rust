(* 1033: Top-K Elements with Binary Heap *)
(* OCaml has no built-in heap — we use sorted insertion or a manual heap *)

(* Approach 1: Sort and take top-k *)
let top_k_sort k lst =
  lst |> List.sort (fun a b -> compare b a) |> List.filteri (fun i _ -> i < k)

let sort_approach () =
  let data = [3; 1; 4; 1; 5; 9; 2; 6; 5; 3] in
  let top3 = top_k_sort 3 data in
  assert (top3 = [9; 6; 5])

(* Approach 2: Maintain a min-heap of size k using a sorted list *)
(* Keep only k largest elements *)
let top_k_linear k lst =
  let insert_bounded heap x =
    if List.length heap < k then
      List.sort compare (x :: heap)
    else
      let min_val = List.hd heap in
      if x > min_val then
        List.sort compare (x :: List.tl heap)
      else
        heap
  in
  List.fold_left (fun heap x -> insert_bounded heap x) [] lst

let bounded_heap_approach () =
  let data = [3; 1; 4; 1; 5; 9; 2; 6; 5; 3] in
  let top3 = top_k_linear 3 data |> List.rev in
  assert (top3 = [9; 6; 5])

(* Approach 3: Top-k with key function *)
let top_k_by k key_fn lst =
  lst
  |> List.map (fun x -> (key_fn x, x))
  |> List.sort (fun (a, _) (b, _) -> compare b a)
  |> List.filteri (fun i _ -> i < k)
  |> List.map snd

let top_k_by_test () =
  let words = ["hi"; "hello"; "hey"; "howdy"; "h"] in
  let longest3 = top_k_by 3 String.length words in
  assert (List.length longest3 = 3);
  assert (List.hd longest3 = "howdy")

let () =
  sort_approach ();
  bounded_heap_approach ();
  top_k_by_test ();
  Printf.printf "✓ All tests passed\n"
