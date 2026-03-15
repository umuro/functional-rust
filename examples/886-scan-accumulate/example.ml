(* Example 092: Scan / Accumulate *)
(* Running fold: OCaml custom → Rust scan *)

(* Approach 1: Manual scan implementation *)
let scan f init lst =
  let rec aux acc state = function
    | [] -> List.rev acc
    | x :: rest ->
      let new_state = f state x in
      aux (new_state :: acc) new_state rest
  in
  aux [init] init lst

let running_sum lst = scan ( + ) 0 lst
let running_max lst =
  match lst with
  | [] -> []
  | x :: rest -> scan max x rest

(* Approach 2: Scan without initial value *)
let scan1 f = function
  | [] -> []
  | x :: rest ->
    let rec aux acc state = function
      | [] -> List.rev acc
      | y :: rest ->
        let new_state = f state y in
        aux (new_state :: acc) new_state rest
    in
    aux [x] x rest

(* Approach 3: Practical applications *)
let running_average lst =
  let _, result = List.fold_left (fun (i, acc) x ->
    let n = i + 1 in
    let avg = (float_of_int (List.fold_left (+) 0 (List.filteri (fun j _ -> j < n) lst))) /. float_of_int n in
    (n, acc @ [avg])
  ) (0, []) lst in
  result

let balance_history transactions =
  scan ( + ) 0 transactions

let is_monotonic lst =
  let diffs = List.map2 (fun a b -> b - a) (List.filteri (fun i _ -> i < List.length lst - 1) lst)
    (List.tl lst) in
  List.for_all (fun d -> d >= 0) diffs || List.for_all (fun d -> d <= 0) diffs

(* Tests *)
let () =
  assert (running_sum [1;2;3;4;5] = [0;1;3;6;10;15]);

  assert (scan1 ( + ) [1;2;3;4;5] = [1;3;6;10;15]);

  assert (balance_history [100; -30; 50; -20] = [0; 100; 70; 120; 100]);

  assert (is_monotonic [1;2;3;4;5]);
  assert (is_monotonic [5;4;3;2;1]);
  assert (not (is_monotonic [1;3;2;4]));

  Printf.printf "✓ All tests passed\n"
