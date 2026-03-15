(* 939: Scan Left — Running Accumulation

   scan_left returns all intermediate results of a fold operation.
   Like fold_left, but keeps the running state at each step.

   OCaml's stdlib doesn't have scan_left, but Seq has map_accumulate (4.14+).
   We show the custom implementation and the Seq version. *)

(* ── Custom scan_left ────────────────────────────────────────────────────── *)

(* scan_left init list f → [init; f init x0; f (f init x0) x1; ...] *)
let scan_left init lst f =
  let (_, result) =
    List.fold_left
      (fun (acc, history) x ->
        let acc' = f acc x in
        (acc', acc' :: history))
      (init, [init])
      lst
  in
  List.rev result

(* Running sum: scan_left 0 [1;2;3;4;5] (+) = [0;1;3;6;10;15] *)
let running_sum nums = scan_left 0 nums ( + )

(* Running max *)
let running_max nums = scan_left min_int nums max

(* Running product *)
let running_product nums = scan_left 1 nums ( * )

(* ── Using Seq.scan (if available via map_accumulate) ────────────────────── *)

(* Build running sums via Seq for a lazy version *)
let running_sum_seq nums =
  let result = ref [0] in
  let acc    = ref 0 in
  List.iter (fun x ->
    acc := !acc + x;
    result := !acc :: !result
  ) nums;
  List.rev !result

(* ── scan_right — right-to-left scan ────────────────────────────────────── *)

let scan_right lst init f =
  let (_, result) =
    List.fold_right
      (fun x (acc, history) ->
        let acc' = f x acc in
        (acc', acc' :: history))
      lst
      (init, [init])
  in
  result

let () =
  (* running_sum *)
  assert (running_sum [1; 2; 3; 4; 5] = [0; 1; 3; 6; 10; 15]);
  assert (running_sum [] = [0]);

  (* running_max *)
  let rm = running_max [3; 1; 4; 1; 5; 9; 2; 6] in
  (* first element is min_int (init), rest should be running max *)
  let tail = List.tl rm in
  assert (tail = [3; 3; 4; 4; 5; 9; 9; 9]);

  (* generic scan_left with string concatenation *)
  let result = scan_left "" ["hello"; " "; "world"] (fun acc s -> acc ^ s) in
  assert (result = [""; "hello"; "hello "; "hello world"]);

  (* running_product *)
  assert (running_product [1; 2; 3; 4] = [1; 1; 2; 6; 24]);

  (* running_sum_seq matches scan_left version *)
  assert (running_sum_seq [1; 2; 3; 4; 5] = running_sum [1; 2; 3; 4; 5]);

  (* scan_right *)
  let sr = scan_right [1; 2; 3] 0 ( + ) in
  (* fold_right: f 3 0 = 3; f 2 3 = 5; f 1 5 = 6 *)
  assert (List.hd sr = 6);

  print_endline "939-scan-left: all tests passed"
