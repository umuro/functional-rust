(* Sweep Line Algorithm in OCaml *)

type interval = { lo: float; hi: float }

(* Event type: 0 = END (process before START at same x), 1 = START *)
type event = { x: float; kind: int; (* 0=end, 1=start *) }

let make_events (intervals : interval list) : event list =
  List.concat_map (fun iv ->
    [{ x = iv.lo; kind = 1 }; { x = iv.hi; kind = 0 }]
  ) intervals
  |> List.sort (fun a b ->
    let cx = compare a.x b.x in
    if cx <> 0 then cx else compare a.kind b.kind)  (* END before START *)

(* Maximum overlap depth: max # of intervals active at any point *)
let max_overlap (intervals : interval list) : int =
  let events = make_events intervals in
  let active = ref 0 and best = ref 0 in
  List.iter (fun ev ->
    if ev.kind = 1 then begin incr active; if !active > !best then best := !active end
    else decr active
  ) events;
  !best

(* Total length of union of intervals *)
let union_length (intervals : interval list) : float =
  let events = make_events intervals in
  let active = ref 0 and total = ref 0.0 and prev_x = ref 0.0 in
  List.iter (fun ev ->
    if !active > 0 then total := !total +. (ev.x -. !prev_x);
    prev_x := ev.x;
    if ev.kind = 1 then incr active else decr active
  ) events;
  !total

(* All points with maximum overlap — just return max value here *)
let () =
  let ivs = [
    {lo=1.0; hi=4.0};
    {lo=2.0; hi=6.0};
    {lo=3.0; hi=5.0};
    {lo=7.0; hi=9.0};
  ] in
  Printf.printf "Intervals: [1,4] [2,6] [3,5] [7,9]\n";
  Printf.printf "Max overlap depth: %d  (expected 3 at [3,4])\n" (max_overlap ivs);
  Printf.printf "Union length: %.1f  (expected 7.0: [1,6]∪[7,9])\n" (union_length ivs);

  (* Touching intervals — should not count as overlapping *)
  let touching = [{lo=0.0;hi=1.0}; {lo=1.0;hi=2.0}; {lo=2.0;hi=3.0}] in
  Printf.printf "\nTouching intervals [0,1] [1,2] [2,3]:\n";
  Printf.printf "Max overlap: %d  (expected 1)\n" (max_overlap touching);
  Printf.printf "Union length: %.1f  (expected 3.0)\n" (union_length touching)
