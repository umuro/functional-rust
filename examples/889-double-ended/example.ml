(* Example 095: Double-Ended Iterator *)
(* Iterate from both ends *)

(* Approach 1: List reversal for back-iteration *)
let take_last n lst =
  let len = List.length lst in
  if n >= len then lst
  else List.filteri (fun i _ -> i >= len - n) lst

let last = function
  | [] -> None
  | lst -> Some (List.nth lst (List.length lst - 1))

(* Approach 2: Array-based bidirectional access *)
let palindrome_check lst =
  let arr = Array.of_list lst in
  let n = Array.length arr in
  let rec aux i j =
    if i >= j then true
    else arr.(i) = arr.(j) && aux (i + 1) (j - 1)
  in
  aux 0 (n - 1)

let interleave_ends lst =
  let arr = Array.of_list lst in
  let n = Array.length arr in
  let result = ref [] in
  let lo = ref 0 and hi = ref (n - 1) in
  while !lo <= !hi do
    result := arr.(!lo) :: !result;
    if !lo <> !hi then result := arr.(!hi) :: !result;
    incr lo;
    decr hi
  done;
  List.rev !result

(* Approach 3: Deque-like operations *)
let trim_while pred lst =
  let trimmed_front = List.to_seq lst |> Seq.drop_while pred |> List.of_seq in
  List.rev trimmed_front |> List.to_seq |> Seq.drop_while pred |> List.of_seq |> List.rev

(* Tests *)
let () =
  assert (take_last 3 [1;2;3;4;5] = [3;4;5]);
  assert (take_last 10 [1;2;3] = [1;2;3]);
  assert (last [1;2;3] = Some 3);
  assert (last [] = None);

  assert (palindrome_check [1;2;3;2;1]);
  assert (not (palindrome_check [1;2;3;4;5]));
  assert (palindrome_check []);
  assert (palindrome_check [1]);

  assert (interleave_ends [1;2;3;4;5] = [1;5;2;4;3]);

  assert (trim_while (fun x -> x = 0) [0;0;1;2;3;0;0] = [1;2;3]);

  Printf.printf "✓ All tests passed\n"
