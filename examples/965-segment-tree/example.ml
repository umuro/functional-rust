(* 965: Segment Tree
   Supports O(log n) range queries and O(log n) point updates.
   Built on a flat array of size 4*n; node at index i has children 2i and 2i+1.
   This example demonstrates range-sum and range-min queries. *)

(* Generic segment tree parameterized by a monoid (identity + combine) *)
type 'a seg_tree = {
  n    : int;          (* number of leaves *)
  data : 'a array;     (* internal nodes; size = 4*n *)
  identity : 'a;
  combine  : 'a -> 'a -> 'a;
}

let create n identity combine =
  assert (n > 0);
  { n; data = Array.make (4 * n) identity; identity; combine }

let build st arr =
  let rec go node l r =
    if l = r then
      st.data.(node) <- arr.(l)
    else begin
      let mid = (l + r) / 2 in
      go (2*node) l mid;
      go (2*node+1) (mid+1) r;
      st.data.(node) <- st.combine st.data.(2*node) st.data.(2*node+1)
    end
  in
  go 1 0 (st.n - 1)

(* Point update: set position i to value *)
let update st i value =
  let rec go node l r =
    if l = r then
      st.data.(node) <- value
    else begin
      let mid = (l + r) / 2 in
      if i <= mid then go (2*node) l mid
      else           go (2*node+1) (mid+1) r;
      st.data.(node) <- st.combine st.data.(2*node) st.data.(2*node+1)
    end
  in
  go 1 0 (st.n - 1)

(* Range query [ql, qr] inclusive *)
let query st ql qr =
  let rec go node l r =
    if ql <= l && r <= qr then
      st.data.(node)                         (* segment fully inside query *)
    else if qr < l || r < ql then
      st.identity                            (* segment fully outside *)
    else begin
      let mid = (l + r) / 2 in
      st.combine (go (2*node) l mid) (go (2*node+1) (mid+1) r)
    end
  in
  go 1 0 (st.n - 1)

let () =
  let arr = [|1; 3; 5; 7; 9; 11|] in
  let n = Array.length arr in

  (* --- Sum segment tree --- *)
  let sum_tree = create n 0 ( + ) in
  build sum_tree arr;

  Printf.printf "Sum queries:\n";
  Printf.printf "  [0,5] = %d\n" (query sum_tree 0 5);  (* 36 *)
  Printf.printf "  [1,3] = %d\n" (query sum_tree 1 3);  (* 15 *)
  Printf.printf "  [2,4] = %d\n" (query sum_tree 2 4);  (* 21 *)

  update sum_tree 3 100;
  Printf.printf "After update idx=3 to 100:\n";
  Printf.printf "  [0,5] = %d\n" (query sum_tree 0 5);  (* 129 *)
  Printf.printf "  [1,3] = %d\n" (query sum_tree 1 3);  (* 108 *)

  (* --- Min segment tree --- *)
  let min_tree = create n max_int min in
  build min_tree arr;

  Printf.printf "\nMin queries:\n";
  Printf.printf "  [0,5] = %d\n" (query min_tree 0 5);  (* 1 *)
  Printf.printf "  [2,5] = %d\n" (query min_tree 2 5);  (* 5 *)
  Printf.printf "  [3,4] = %d\n" (query min_tree 3 4);  (* 7 *)

  (* --- Product segment tree --- *)
  let prod_tree = create 4 1 ( * ) in
  build prod_tree [|2; 3; 4; 5|];
  Printf.printf "\nProduct [0,3] = %d\n" (query prod_tree 0 3);  (* 120 *)
  Printf.printf "Product [1,2] = %d\n" (query prod_tree 1 2)     (* 12 *)
