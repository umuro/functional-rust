(* 1043: Interval Map — sorted list of non-overlapping [lo, hi) -> value
   OCaml's Map (sorted by key) mirrors Rust's BTreeMap for range lookups. *)

(* Each entry: (lo, hi, value) — sorted by lo, non-overlapping *)
(* We use a sorted association list for simplicity and clarity *)

type 'a interval_map = (int * int * 'a) list  (* (lo, hi, value) sorted by lo *)

let empty_im : 'a interval_map = []

(* Insert [lo, hi) -> value, removing any overlapping intervals *)
let insert lo hi value im =
  assert (lo < hi);
  (* Keep intervals that do NOT overlap with [lo, hi) *)
  let non_overlapping = List.filter (fun (l, h, _) ->
    (* Overlaps if l < hi && h > lo *)
    not (l < hi && h > lo)
  ) im in
  (* Insert in sorted order *)
  let rec ins = function
    | [] -> [(lo, hi, value)]
    | ((l, _, _) as hd) :: rest ->
      if lo < l then (lo, hi, value) :: hd :: rest
      else hd :: ins rest
  in
  ins non_overlapping

(* Point query: find the interval containing point, if any *)
let query point im =
  (* Find last interval where lo <= point, then check hi > point *)
  let rec aux best = function
    | [] -> best
    | (lo, hi, v) :: rest ->
      if lo > point then best
      else aux (if lo <= point && point < hi then Some v else best) rest
  in
  aux None im

(* List all intervals *)
let intervals im = im

let len im = List.length im

let () =
  let im = ref empty_im in
  im := insert 0 10 "low" !im;
  im := insert 10 20 "mid" !im;
  im := insert 20 30 "high" !im;

  assert (query 5  !im = Some "low");
  assert (query 15 !im = Some "mid");
  assert (query 25 !im = Some "high");
  assert (query 30 !im = None);
  assert (query (-1) !im = None);
  assert (len !im = 3);

  (* Overlap: [5,15) replaces [0,10) *)
  let im2 = ref empty_im in
  im2 := insert 0 10 "a" !im2;
  im2 := insert 5 15 "b" !im2;
  assert (query 7  !im2 = Some "b");
  assert (query 12 !im2 = Some "b");

  (* Listing *)
  let im3 = ref empty_im in
  im3 := insert 0 5 "x" !im3;
  im3 := insert 10 20 "y" !im3;
  let ivs = intervals !im3 in
  assert (List.length ivs = 2);
  assert (List.nth ivs 0 = (0, 5, "x"));
  assert (List.nth ivs 1 = (10, 20, "y"));

  (* Boundary: [0,10) includes 0..9, excludes 10 *)
  let im4 = ref empty_im in
  im4 := insert 0 10 "a" !im4;
  assert (query 0 !im4 = Some "a");
  assert (query 9 !im4 = Some "a");
  assert (query 10 !im4 = None);

  (* Adjacent intervals *)
  let im5 = ref empty_im in
  im5 := insert 0 5 "a" !im5;
  im5 := insert 5 10 "b" !im5;
  assert (query 4 !im5 = Some "a");
  assert (query 5 !im5 = Some "b");

  Printf.printf "All interval-map tests passed.\n"
