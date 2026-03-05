(* Algorithm Complexity Guide in OCaml — reference implementations *)

(* O(1) — Constant time: direct access *)
let constant_example arr i = Array.get arr i

(* O(log n) — Logarithmic: binary search *)
let rec binary_search arr target lo hi =
  if lo > hi then None
  else
    let mid = (lo + hi) / 2 in
    if arr.(mid) = target then Some mid
    else if arr.(mid) < target then binary_search arr target (mid + 1) hi
    else binary_search arr target lo (mid - 1)

(* O(n) — Linear: find maximum *)
let linear_max arr =
  Array.fold_left max min_int arr

(* O(n log n) — Linearithmic: merge sort *)
let rec merge_sort = function
  | [] | [_] as xs -> xs
  | xs ->
    let n = List.length xs in
    let l = List.filteri (fun i _ -> i < n/2) xs in
    let r = List.filteri (fun i _ -> i >= n/2) xs in
    let rec merge a b = match a, b with
      | [], b -> b | a, [] -> a
      | x::xs, y::ys -> if x <= y then x :: merge xs b else y :: merge a ys
    in
    merge (merge_sort l) (merge_sort r)

(* O(n²) — Quadratic: insertion sort *)
let insertion_sort arr =
  let n = Array.length arr in
  for i = 1 to n - 1 do
    let key = arr.(i) in
    let j = ref (i - 1) in
    while !j >= 0 && arr.(!j) > key do
      arr.(!j + 1) <- arr.(!j);
      decr j
    done;
    arr.(!j + 1) <- key
  done

(* Master theorem examples *)
(* T(n) = 2T(n/2) + O(n)  → O(n log n)  [merge sort] *)
(* T(n) = 1T(n/2) + O(1)  → O(log n)    [binary search] *)
(* T(n) = 2T(n/2) + O(1)  → O(n)        [tree traversal] *)
(* T(n) = 2T(n-1) + O(1)  → O(2^n)      [naive Fibonacci] *)

let () =
  let arr = Array.init 20 (fun i -> i * 2) in
  Printf.printf "O(1) — arr[5] = %d\n" (constant_example arr 5);
  Printf.printf "O(log n) — binary_search(10) = %s\n"
    (match binary_search arr 10 0 19 with Some i -> string_of_int i | None -> "None");
  Printf.printf "O(n) — max of [0,2,..,38] = %d\n" (linear_max arr);
  Printf.printf "O(n log n) — merge_sort([3,1,4,1,5,9]) = [%s]\n"
    (String.concat "," (List.map string_of_int (merge_sort [3;1;4;1;5;9])));

  (* Empirical timing example: show n² vs n log n difference *)
  let sizes = [100; 1000; 10000] in
  List.iter (fun n ->
    let a = Array.init n (fun i -> n - i) in
    insertion_sort a;
    Printf.printf "insertion_sort(%d): sorted = %b\n" n
      (Array.for_all2 (<=) (Array.sub a 0 (n-1)) (Array.sub a 1 (n-1)))
  ) sizes
