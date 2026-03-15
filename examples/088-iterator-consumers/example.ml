(* 088: Iterator Consumers *)

let range a b = Seq.init (b - a) (fun i -> i + a)

(* Approach 1: Basic consumers *)
let seq_sum s = Seq.fold_left ( + ) 0 s
let seq_product s = Seq.fold_left ( * ) 1 s
let seq_count s = Seq.fold_left (fun n _ -> n + 1) 0 s
let seq_collect s = List.of_seq s

(* Approach 2: for_each with side effects *)
let seq_for_each f s = Seq.iter f s

(* Approach 3: Custom consumers *)
let seq_min s =
  Seq.fold_left (fun acc x ->
    match acc with None -> Some x | Some m -> Some (min m x)
  ) None s

let seq_max s =
  Seq.fold_left (fun acc x ->
    match acc with None -> Some x | Some m -> Some (max m x)
  ) None s

let seq_any pred s =
  let found = ref false in
  Seq.iter (fun x -> if pred x then found := true) s;
  !found

let seq_all pred s =
  let ok = ref true in
  Seq.iter (fun x -> if not (pred x) then ok := false) s;
  !ok

(* Tests *)
let () =
  assert (seq_sum (range 1 6) = 15);
  assert (seq_product (range 1 6) = 120);
  assert (seq_count (range 0 10) = 10);
  assert (seq_collect (range 0 3) = [0; 1; 2]);
  assert (seq_min (List.to_seq [3; 1; 4; 1; 5]) = Some 1);
  assert (seq_max (List.to_seq [3; 1; 4; 1; 5]) = Some 5);
  assert (seq_any (fun x -> x > 3) (List.to_seq [1; 2; 3; 4]));
  assert (not (seq_any (fun x -> x > 10) (List.to_seq [1; 2; 3])));
  assert (seq_all (fun x -> x > 0) (List.to_seq [1; 2; 3]));
  assert (not (seq_all (fun x -> x > 2) (List.to_seq [1; 2; 3])));
  Printf.printf "✓ All tests passed\n"
