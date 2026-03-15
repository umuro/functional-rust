(* 966: Fenwick Tree (Binary Indexed Tree) *)
(* Point update + prefix sum in O(log n). Uses 1-based indexing internally. *)

type fenwick = {
  n: int;
  tree: int array;  (* 1-indexed *)
}

let create n = { n; tree = Array.make (n + 1) 0 }

(* lowbit: lowest set bit of i *)
let lowbit i = i land (-i)

(* Point update: add delta to position i (1-indexed externally) *)
let update fw i delta =
  let i = ref (i + 1) in  (* convert to 1-indexed *)
  while !i <= fw.n do
    fw.tree.(!i) <- fw.tree.(!i) + delta;
    i := !i + lowbit !i
  done

(* Prefix sum [0, i] inclusive (0-indexed externally) *)
let prefix_sum fw i =
  let i = ref (i + 1) in  (* convert to 1-indexed *)
  let s = ref 0 in
  while !i > 0 do
    s := !s + fw.tree.(!i);
    i := !i - lowbit !i
  done;
  !s

(* Range sum [l, r] inclusive (0-indexed) *)
let range_sum fw l r =
  if l = 0 then prefix_sum fw r
  else prefix_sum fw r - prefix_sum fw (l - 1)

(* Build from array *)
let build arr =
  let n = Array.length arr in
  let fw = create n in
  Array.iteri (fun i v -> update fw i v) arr;
  fw

let () =
  let arr = [| 1; 3; 5; 7; 9; 11 |] in
  let fw = build arr in

  assert (prefix_sum fw 0 = 1);
  assert (prefix_sum fw 2 = 9);   (* 1+3+5 *)
  assert (prefix_sum fw 5 = 36);  (* total *)

  assert (range_sum fw 0 2 = 9);
  assert (range_sum fw 2 4 = 21);
  assert (range_sum fw 1 3 = 15);

  (* Update arr[2] += 5: now arr[2] = 10 instead of 5 *)
  update fw 2 5;
  assert (prefix_sum fw 5 = 41);
  assert (range_sum fw 0 2 = 14);
  assert (range_sum fw 2 4 = 26);

  Printf.printf "✓ All tests passed\n"
