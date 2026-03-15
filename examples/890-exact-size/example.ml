(* Example 096: ExactSizeIterator *)
(* When you know the length upfront *)

(* Approach 1: List.length — always available *)
let process_with_progress lst =
  let total = List.length lst in
  List.mapi (fun i x ->
    Printf.sprintf "[%d/%d] Processing %d" (i + 1) total x
  ) lst

(* Approach 2: Array-based known-size operations *)
let array_chunks_exact n arr =
  let len = Array.length arr in
  let num_chunks = len / n in
  Array.init num_chunks (fun i ->
    Array.sub arr (i * n) n
  )

let progress_bar completed total width =
  let filled = completed * width / total in
  let empty = width - filled in
  Printf.sprintf "[%s%s] %d/%d"
    (String.make filled '#') (String.make empty '.') completed total

(* Approach 3: Pre-allocate based on known size *)
let map_preallocated f lst =
  let len = List.length lst in
  let arr = Array.make len (f (List.hd lst)) in
  List.iteri (fun i x -> arr.(i) <- f x) lst;
  Array.to_list arr

let parallel_process lst =
  let n = List.length lst in
  let mid = n / 2 in
  let first_half = List.filteri (fun i _ -> i < mid) lst in
  let second_half = List.filteri (fun i _ -> i >= mid) lst in
  (first_half, second_half)

(* Tests *)
let () =
  let progress = process_with_progress [10; 20; 30] in
  assert (List.hd progress = "[1/3] Processing 10");
  assert (List.length progress = 3);

  let chunks = array_chunks_exact 2 [|1;2;3;4;5;6|] in
  assert (Array.length chunks = 3);
  assert (chunks.(0) = [|1;2|]);

  assert (progress_bar 3 10 20 = "[######..............] 3/10");

  let mapped = map_preallocated (fun x -> x * 2) [1;2;3] in
  assert (mapped = [2;4;6]);

  let (a, b) = parallel_process [1;2;3;4;5;6] in
  assert (a = [1;2;3]);
  assert (b = [4;5;6]);

  Printf.printf "✓ All tests passed\n"
