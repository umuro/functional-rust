(* Randomised Quickselect in OCaml *)

(* Simple LCG pseudo-random number generator (no dependencies) *)
let state = ref 42

let rand_int n =
  state := (!state * 1664525 + 1013904223) land 0x7fffffff;
  !state mod n

(* Partition: rearrange arr[lo..hi] around pivot, return pivot index *)
let partition (arr : int array) (lo hi : int) : int =
  let pivot_idx = lo + rand_int (hi - lo + 1) in
  let pivot = arr.(pivot_idx) in
  (* Move pivot to end *)
  let tmp = arr.(pivot_idx) in arr.(pivot_idx) <- arr.(hi); arr.(hi) <- tmp;
  let store = ref lo in
  for i = lo to hi - 1 do
    if arr.(i) < pivot then begin
      let tmp = arr.(!store) in arr.(!store) <- arr.(i); arr.(i) <- tmp;
      incr store
    end
  done;
  (* Move pivot to final position *)
  let tmp = arr.(!store) in arr.(!store) <- arr.(hi); arr.(hi) <- tmp;
  !store

(* Quickselect: find the k-th smallest (0-indexed) in arr[lo..hi] *)
let rec quickselect (arr : int array) (lo hi k : int) : int =
  if lo = hi then arr.(lo)
  else
    let p = partition arr lo hi in
    if p = k then arr.(p)
    else if p > k then quickselect arr lo (p - 1) k
    else quickselect arr (p + 1) hi k

(* Public API: k-th smallest (1-indexed) in the array *)
let kth_smallest (arr : int array) (k : int) : int =
  let copy = Array.copy arr in
  quickselect copy 0 (Array.length copy - 1) (k - 1)

(* Median of array *)
let median (arr : int array) : float =
  let n = Array.length arr in
  if n mod 2 = 1 then
    float_of_int (kth_smallest arr ((n + 1) / 2))
  else
    (float_of_int (kth_smallest arr (n / 2)) +.
     float_of_int (kth_smallest arr (n / 2 + 1))) /. 2.0

let () =
  let arr = [| 7; 10; 4; 3; 20; 15 |] in
  Printf.printf "Array: [7, 10, 4, 3, 20, 15]\n";
  for k = 1 to Array.length arr do
    Printf.printf "  %d-th smallest: %d\n" k (kth_smallest arr k)
  done;
  Printf.printf "Median: %.1f\n" (median arr);

  let arr2 = [| 1; 2; 3; 4; 5; 6; 7; 8; 9; 10 |] in
  Printf.printf "\n5th smallest of 1..10: %d (expected 5)\n" (kth_smallest arr2 5)
