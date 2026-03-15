(* 1047: Flat Binary Tree in Array *)
(* Children of node i: left = 2i+1, right = 2i+2, parent = (i-1)/2 *)

(* Approach 1: Basic flat tree operations *)
let left_child i = 2 * i + 1
let right_child i = 2 * i + 2
let parent i = (i - 1) / 2

let basic_tree () =
  (*       1
         /   \
        2     3
       / \   /
      4   5 6      *)
  let tree = [|1; 2; 3; 4; 5; 6|] in
  let n = Array.length tree in
  (* Root *)
  assert (tree.(0) = 1);
  (* Children of root *)
  assert (tree.(left_child 0) = 2);
  assert (tree.(right_child 0) = 3);
  (* Children of node 1 (value=2) *)
  assert (tree.(left_child 1) = 4);
  assert (tree.(right_child 1) = 5);
  (* Parent of node 5 (index=4) *)
  assert (tree.(parent 4) = 2);
  (* Leaf check *)
  let is_leaf i = left_child i >= n in
  assert (is_leaf 3);
  assert (not (is_leaf 0))

(* Approach 2: Level-order traversal *)
let level_order tree =
  let n = Array.length tree in
  let levels = ref [] in
  let i = ref 0 in
  let level_size = ref 1 in
  while !i < n do
    let level = ref [] in
    for j = 0 to min (!level_size - 1) (n - !i - 1) do
      level := tree.(!i + j) :: !level
    done;
    levels := List.rev !level :: !levels;
    i := !i + !level_size;
    level_size := !level_size * 2
  done;
  List.rev !levels

let level_order_test () =
  let tree = [|1; 2; 3; 4; 5; 6; 7|] in
  let levels = level_order tree in
  assert (levels = [[1]; [2; 3]; [4; 5; 6; 7]])

(* Approach 3: Build max-heap (heapify) *)
let swap arr i j =
  let tmp = arr.(i) in
  arr.(i) <- arr.(j);
  arr.(j) <- tmp

let rec sift_down arr n i =
  let largest = ref i in
  let l = left_child i in
  let r = right_child i in
  if l < n && arr.(l) > arr.(!largest) then largest := l;
  if r < n && arr.(r) > arr.(!largest) then largest := r;
  if !largest <> i then begin
    swap arr i !largest;
    sift_down arr n !largest
  end

let heapify arr =
  let n = Array.length arr in
  for i = n / 2 - 1 downto 0 do
    sift_down arr n i
  done

let heap_test () =
  let arr = [|3; 1; 4; 1; 5; 9; 2; 6|] in
  heapify arr;
  (* After heapify, root should be max *)
  assert (arr.(0) = 9);
  (* Heap property: parent >= children *)
  let n = Array.length arr in
  for i = 1 to n - 1 do
    assert (arr.(parent i) >= arr.(i))
  done

let () =
  basic_tree ();
  level_order_test ();
  heap_test ();
  Printf.printf "✓ All tests passed\n"
