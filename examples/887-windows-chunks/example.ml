(* Example 093: Windows and Chunks *)
(* Sliding window algorithms *)

(* Approach 1: Windows (sliding window) *)
let windows n lst =
  let arr = Array.of_list lst in
  let len = Array.length arr in
  if n > len then []
  else List.init (len - n + 1) (fun i ->
    Array.to_list (Array.sub arr i n))

let moving_average n lst =
  let ws = windows n (List.map float_of_int lst) in
  List.map (fun w ->
    List.fold_left (+.) 0.0 w /. float_of_int n
  ) ws

let pairwise_diff lst =
  let ws = windows 2 lst in
  List.map (function [a; b] -> b - a | _ -> 0) ws

(* Approach 2: Chunks (non-overlapping) *)
let chunks n lst =
  let rec aux acc current count = function
    | [] -> List.rev (if current = [] then acc else List.rev current :: acc)
    | x :: rest ->
      if count = n then
        aux (List.rev current :: acc) [x] 1 rest
      else
        aux acc (x :: current) (count + 1) rest
  in
  aux [] [] 0 lst

let chunks_exact n lst =
  let all = chunks n lst in
  List.filter (fun chunk -> List.length chunk = n) all

(* Approach 3: Practical algorithms *)
let local_maxima lst =
  windows 3 lst
  |> List.filter (function [a; b; c] -> b > a && b > c | _ -> false)
  |> List.map (function [_; b; _] -> b | _ -> 0)

let contains_pattern pattern lst =
  let n = List.length pattern in
  windows n lst |> List.exists (fun w -> w = pattern)

(* Tests *)
let () =
  assert (windows 3 [1;2;3;4;5] = [[1;2;3]; [2;3;4]; [3;4;5]]);
  assert (windows 2 [1;2;3] = [[1;2]; [2;3]]);
  assert (windows 5 [1;2;3] = []);

  let avg = moving_average 3 [1;2;3;4;5] in
  assert (abs_float (List.hd avg -. 2.0) < 0.001);

  assert (pairwise_diff [1;3;6;10] = [2; 3; 4]);

  assert (chunks 2 [1;2;3;4;5] = [[1;2]; [3;4]; [5]]);
  assert (chunks_exact 2 [1;2;3;4;5] = [[1;2]; [3;4]]);

  assert (local_maxima [1;3;2;5;4;6;1] = [3; 5; 6]);
  assert (contains_pattern [3;4;5] [1;2;3;4;5;6]);
  assert (not (contains_pattern [3;5] [1;2;3;4;5;6]));

  Printf.printf "✓ All tests passed\n"
