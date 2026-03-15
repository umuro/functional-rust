(* 1058: Longest Increasing Subsequence
   O(n^2) DP, O(n log n) patience sorting, and functional fold. *)

(* Approach 1: O(n^2) DP *)
let lis_dp arr =
  let n = Array.length arr in
  if n = 0 then 0
  else begin
    let dp = Array.make n 1 in
    for i = 1 to n - 1 do
      for j = 0 to i - 1 do
        if arr.(j) < arr.(i) && dp.(j) + 1 > dp.(i) then
          dp.(i) <- dp.(j) + 1
      done
    done;
    Array.fold_left max 0 dp
  end

(* Approach 2: O(n log n) patience sorting with binary search *)
let lis_patience arr =
  (* tails.(i) = smallest tail of IS of length i+1 *)
  let tails = ref [||] in
  Array.iter (fun x ->
    (* Binary search for leftmost position where tails.(pos) >= x *)
    let lo = ref 0 and hi = ref (Array.length !tails) in
    while !lo < !hi do
      let mid = (!lo + !hi) / 2 in
      if !tails.(mid) < x then lo := mid + 1
      else hi := mid
    done;
    let pos = !lo in
    if pos = Array.length !tails then
      tails := Array.append !tails [|x|]
    else
      !tails.(pos) <- x
  ) arr;
  Array.length !tails

(* Approach 3: Functional fold — builds tails list purely *)
let lis_fold arr =
  let insert_or_replace tails x =
    (* Find leftmost position >= x *)
    let rec find = function
      | [] -> [x]
      | h :: rest ->
        if h >= x then x :: rest   (* replace h with x *)
        else h :: find rest
    in
    find tails
  in
  let tails = Array.fold_left insert_or_replace [] arr in
  List.length tails

let () =
  let cases = [
    ([|10;9;2;5;3;7;101;18|], 4);
    ([|0;1;0;3;2;3|],          4);
    ([|7;7;7;7|],              1);
  ] in
  List.iter (fun (arr, expected) ->
    assert (lis_dp      arr = expected);
    assert (lis_patience arr = expected);
    assert (lis_fold     arr = expected)
  ) cases;
  Printf.printf "All LIS tests passed.\n"
