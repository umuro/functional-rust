(* 965: Segment Tree for Range Sum Queries *)
(* 1-indexed internal nodes, supports point update + range query in O(log n) *)

type segment_tree = {
  n: int;
  tree: int array;  (* 1-indexed, size 4*n *)
}

let create n = { n; tree = Array.make (4 * n) 0 }

(* Build from array *)
let rec build st node lo hi arr =
  if lo = hi then
    st.tree.(node) <- arr.(lo)
  else begin
    let mid = (lo + hi) / 2 in
    build st (2*node) lo mid arr;
    build st (2*node+1) (mid+1) hi arr;
    st.tree.(node) <- st.tree.(2*node) + st.tree.(2*node+1)
  end

(* Point update: set position pos to value *)
let rec update st node lo hi pos value =
  if lo = hi then
    st.tree.(node) <- value
  else begin
    let mid = (lo + hi) / 2 in
    if pos <= mid then update st (2*node) lo mid pos value
    else update st (2*node+1) (mid+1) hi pos value;
    st.tree.(node) <- st.tree.(2*node) + st.tree.(2*node+1)
  end

(* Range sum query [l, r] *)
let rec query st node lo hi l r =
  if r < lo || hi < l then 0
  else if l <= lo && hi <= r then st.tree.(node)
  else begin
    let mid = (lo + hi) / 2 in
    query st (2*node) lo mid l r +
    query st (2*node+1) (mid+1) hi l r
  end

let st_update st pos value = update st 1 0 (st.n-1) pos value
let st_query st l r = query st 1 0 (st.n-1) l r
let st_build st arr = build st 1 0 (st.n-1) arr

let () =
  let arr = [| 1; 3; 5; 7; 9; 11 |] in
  let n = Array.length arr in
  let st = create n in
  st_build st arr;

  (* Sum of entire array *)
  assert (st_query st 0 (n-1) = 36);

  (* Range queries *)
  assert (st_query st 0 2 = 9);   (* 1+3+5 *)
  assert (st_query st 2 4 = 21);  (* 5+7+9 *)
  assert (st_query st 1 3 = 15);  (* 3+5+7 *)

  (* Point update *)
  st_update st 2 10;   (* arr[2] = 10 instead of 5 *)
  assert (st_query st 0 (n-1) = 41);  (* 36 - 5 + 10 *)
  assert (st_query st 0 2 = 14);  (* 1+3+10 *)
  assert (st_query st 2 4 = 26);  (* 10+7+9 *)

  Printf.printf "✓ All tests passed\n"
