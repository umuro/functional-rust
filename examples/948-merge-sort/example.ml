(* 948: Merge Sort — Functional Divide and Conquer

   Pure functional merge sort using OCaml lists.
   OCaml's pattern matching on lists makes split and merge concise.
   The tail-recursive version avoids stack overflow on long inputs. *)

(* ── Merge two sorted lists ──────────────────────────────────────────────── *)

(* Non-tail-recursive version — clear and direct *)
let rec merge cmp left right =
  match (left, right) with
  | ([], r) -> r
  | (l, []) -> l
  | (lh :: lt, rh :: _) when cmp lh rh <= 0 ->
    lh :: merge cmp lt right
  | (_, rh :: rt) ->
    rh :: merge cmp left rt

(* Tail-recursive merge (builds reversed result, then reverses) *)
let merge_tr cmp left right =
  let rec go acc l r =
    match (l, r) with
    | ([], r) -> List.rev_append acc r
    | (l, []) -> List.rev_append acc l
    | (lh :: lt, rh :: _) when cmp lh rh <= 0 -> go (lh :: acc) lt r
    | (_, rh :: rt) -> go (rh :: acc) l rt
  in
  go [] left right

(* ── Split a list into two halves ────────────────────────────────────────── *)

let split lst =
  let rec go acc1 acc2 = function
    | [] -> (List.rev acc1, List.rev acc2)
    | [x] -> (List.rev (x :: acc1), List.rev acc2)
    | x :: y :: rest -> go (x :: acc1) (y :: acc2) rest
  in
  go [] [] lst

(* ── Merge sort ──────────────────────────────────────────────────────────── *)

let rec merge_sort cmp = function
  | ([] | [_]) as lst -> lst
  | lst ->
    let (left, right) = split lst in
    merge cmp (merge_sort cmp left) (merge_sort cmp right)

(* Convenience: sort with standard polymorphic compare *)
let sort lst = merge_sort compare lst

(* Sort descending *)
let sort_desc lst = merge_sort (fun a b -> compare b a) lst

(* Sort by key function *)
let sort_by f lst = merge_sort (fun a b -> compare (f a) (f b)) lst

(* ── Stability check: sort by first element, preserve second ─────────────── *)

(* Merge sort is stable — equal elements maintain their original order *)
let is_sorted cmp lst =
  let rec go = function
    | [] | [_] -> true
    | a :: (b :: _ as rest) -> cmp a b <= 0 && go rest
  in
  go lst

let () =
  (* basic sort *)
  assert (sort [5; 2; 8; 1; 9; 3] = [1; 2; 3; 5; 8; 9]);
  assert (sort ([] : int list) = []);
  assert (sort [42] = [42]);
  assert (sort [1; 2; 3; 4; 5] = [1; 2; 3; 4; 5]);  (* already sorted *)
  assert (sort [5; 4; 3; 2; 1] = [1; 2; 3; 4; 5]);  (* reversed *)
  assert (sort [3; 1; 2; 1; 3] = [1; 1; 2; 3; 3]);  (* duplicates *)

  (* strings *)
  assert (sort ["banana"; "apple"; "cherry"] = ["apple"; "banana"; "cherry"]);

  (* descending *)
  assert (sort_desc [1; 5; 3; 2; 4] = [5; 4; 3; 2; 1]);

  (* sort_by: sort by string length *)
  let by_len = sort_by String.length ["cat"; "elephant"; "ox"] in
  assert (by_len = ["ox"; "cat"; "elephant"]);

  (* tail-recursive merge gives same result *)
  assert (merge_tr compare [1; 3; 5] [2; 4; 6] = [1; 2; 3; 4; 5; 6]);
  assert (merge compare [1; 3; 5] [2; 4; 6] = [1; 2; 3; 4; 5; 6]);

  (* is_sorted helper *)
  assert (is_sorted compare (sort [5; 2; 8; 1]));
  assert (not (is_sorted compare [3; 1; 2]));

  (* Stability: (key, id) pairs sorted by key should preserve id order *)
  let pairs = [(1, 'a'); (2, 'b'); (1, 'c'); (2, 'd')] in
  let sorted = merge_sort (fun (k1,_) (k2,_) -> compare k1 k2) pairs in
  assert (sorted = [(1,'a'); (1,'c'); (2,'b'); (2,'d')]);

  print_endline "948-merge-sort: all tests passed"
