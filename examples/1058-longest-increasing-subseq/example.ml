(* 1058: Longest Increasing Subsequence — O(n log n) Patience Sorting *)

(* Approach 1: O(n^2) DP *)
let lis_dp arr =
  let n = Array.length arr in
  if n = 0 then 0
  else begin
    let dp = Array.make n 1 in
    for i = 1 to n - 1 do
      for j = 0 to i - 1 do
        if arr.(j) < arr.(i) then
          dp.(i) <- max dp.(i) (dp.(j) + 1)
      done
    done;
    Array.fold_left max 0 dp
  end

(* Approach 2: O(n log n) patience sorting with binary search *)
let lis_patience arr =
  let n = Array.length arr in
  if n = 0 then 0
  else begin
    let tails = Array.make n 0 in
    let len = ref 0 in
    for i = 0 to n - 1 do
      (* Binary search for position *)
      let lo = ref 0 and hi = ref !len in
      while !lo < !hi do
        let mid = (!lo + !hi) / 2 in
        if tails.(mid) < arr.(i) then lo := mid + 1
        else hi := mid
      done;
      tails.(!lo) <- arr.(i);
      if !lo = !len then incr len
    done;
    !len
  end

(* Approach 3: Functional with lists *)
let lis_functional arr =
  let binary_search tails x len =
    let lo = ref 0 and hi = ref len in
    while !lo < !hi do
      let mid = (!lo + !hi) / 2 in
      if tails.(mid) < x then lo := mid + 1
      else hi := mid
    done;
    !lo
  in
  let n = Array.length arr in
  if n = 0 then 0
  else begin
    let tails = Array.make n 0 in
    let len = ref 0 in
    Array.iter (fun x ->
      let pos = binary_search tails x !len in
      tails.(pos) <- x;
      if pos = !len then incr len
    ) arr;
    !len
  end

let () =
  assert (lis_dp [|10; 9; 2; 5; 3; 7; 101; 18|] = 4);
  assert (lis_dp [|0; 1; 0; 3; 2; 3|] = 4);
  assert (lis_dp [|7; 7; 7; 7|] = 1);

  assert (lis_patience [|10; 9; 2; 5; 3; 7; 101; 18|] = 4);
  assert (lis_patience [|0; 1; 0; 3; 2; 3|] = 4);
  assert (lis_patience [|7; 7; 7; 7|] = 1);

  assert (lis_functional [|10; 9; 2; 5; 3; 7; 101; 18|] = 4);

  Printf.printf "✓ All tests passed\n"
