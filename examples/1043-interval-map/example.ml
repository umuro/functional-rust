(* 1043: Interval Map — BTreeMap-based Range Storage *)
(* Map non-overlapping intervals to values *)

module IntMap = Map.Make(Int)

(* Store intervals as (start, (end, value)) in a sorted map *)
type 'a interval_map = (int * 'a) IntMap.t

let empty : 'a interval_map = IntMap.empty

(* Insert interval [lo, hi) -> value (overwrites overlapping ranges) *)
let insert lo hi value (im : 'a interval_map) : 'a interval_map =
  (* Remove all intervals that overlap with [lo, hi) *)
  let filtered = IntMap.filter (fun start (stop, _) ->
    stop <= lo || start >= hi  (* keep if completely before or after *)
  ) im in
  IntMap.add lo (hi, value) filtered

(* Query: find value at point *)
let query point (im : 'a interval_map) : 'a option =
  (* Find the largest start <= point *)
  let (below, at_point, _) = IntMap.split point im in
  match at_point with
  | Some (hi, v) when point < hi -> Some v
  | _ ->
    match IntMap.max_binding_opt below with
    | Some (_, (hi, v)) when point < hi -> Some v
    | _ -> None

(* Approach 1: Basic interval operations *)
let basic_ops () =
  let im = empty
    |> insert 0 10 "low"
    |> insert 10 20 "mid"
    |> insert 20 30 "high"
  in
  assert (query 5 im = Some "low");
  assert (query 15 im = Some "mid");
  assert (query 25 im = Some "high");
  assert (query 30 im = None);
  assert (query (-1) im = None)

(* Approach 2: Overlapping insert *)
let overlap_test () =
  let im = empty
    |> insert 0 10 "a"
    |> insert 5 15 "b"  (* overlaps with "a" *)
  in
  (* "b" overwrites the overlapping portion *)
  assert (query 7 im = Some "b");
  assert (query 12 im = Some "b")

(* Approach 3: List all intervals *)
let to_list (im : 'a interval_map) =
  IntMap.fold (fun start (stop, value) acc ->
    (start, stop, value) :: acc
  ) im [] |> List.rev

let list_test () =
  let im = empty
    |> insert 0 5 "x"
    |> insert 10 20 "y"
  in
  let intervals = to_list im in
  assert (List.length intervals = 2);
  let (s, e, v) = List.hd intervals in
  assert (s = 0 && e = 5 && v = "x")

let () =
  basic_ops ();
  overlap_test ();
  list_test ();
  Printf.printf "✓ All tests passed\n"
