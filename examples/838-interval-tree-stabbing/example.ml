(* Interval Tree for Stabbing Queries in OCaml *)

type interval = { lo: float; hi: float; id: int }

(* Each node: median, intervals spanning median (sorted by lo and hi),
   left subtree (all intervals < median), right subtree (all > median) *)
type tree =
  | Empty
  | Node of {
      median  : float;
      by_lo   : interval list;  (* sorted ascending by lo *)
      by_hi   : interval list;  (* sorted descending by hi *)
      left    : tree;
      right   : tree;
    }

let build (intervals : interval list) : tree =
  let rec build_rec ivs =
    match ivs with
    | [] -> Empty
    | _ ->
      (* Median of endpoints *)
      let endpoints = List.concat_map (fun iv -> [iv.lo; iv.hi]) ivs in
      let sorted_ep = List.sort compare endpoints in
      let n = List.length sorted_ep in
      let median = List.nth sorted_ep (n / 2) in
      (* Partition: spanning, left, right *)
      let spanning = List.filter (fun iv -> iv.lo <= median && iv.hi >= median) ivs in
      let left_ivs = List.filter (fun iv -> iv.hi < median) ivs in
      let right_ivs = List.filter (fun iv -> iv.lo > median) ivs in
      let by_lo = List.sort (fun a b -> compare a.lo b.lo) spanning in
      let by_hi = List.sort (fun a b -> compare b.hi a.hi) spanning in
      Node {
        median;
        by_lo; by_hi;
        left = build_rec left_ivs;
        right = build_rec right_ivs;
      }
  in
  build_rec intervals

(* Stab query: all intervals containing point x *)
let query (x : float) (tree : tree) : interval list =
  let rec query_rec t acc =
    match t with
    | Empty -> acc
    | Node { median; by_lo; by_hi; left; right } ->
      let acc = if x <= median then begin
        (* Scan by_lo until lo > x *)
        let rec scan = function
          | [] -> acc
          | iv :: rest -> if iv.lo > x then acc else iv :: scan rest
        in
        scan by_lo
      end else begin
        (* Scan by_hi (desc) until hi < x *)
        let rec scan = function
          | [] -> acc
          | iv :: rest -> if iv.hi < x then acc else iv :: scan rest
        in
        scan by_hi
      end in
      let acc = if x < median then query_rec left acc else query_rec right acc in
      acc
  in
  query_rec tree []

let () =
  let ivs = [
    {lo=1.0; hi=5.0; id=1};
    {lo=2.0; hi=8.0; id=2};
    {lo=6.0; hi=10.0; id=3};
    {lo=3.0; hi=7.0; id=4};
    {lo=9.0; hi=12.0; id=5};
  ] in
  let tree = build ivs in
  let queries = [0.0; 3.0; 6.5; 9.5; 15.0] in
  List.iter (fun x ->
    let results = query x tree in
    let ids = List.map (fun iv -> iv.id) results in
    Printf.printf "stab(%.1f): ids=[%s]\n" x
      (String.concat "," (List.map string_of_int (List.sort compare ids)))
  ) queries
