(* 1047: Flat Binary Tree Stored in an Array
   Children of node i: left = 2*i+1, right = 2*i+2, parent = (i-1)/2
   Same arithmetic works in OCaml with a plain array. *)

type 'a flat_tree = { data : 'a array }

let make data = { data }

let left_child  i = 2 * i + 1
let right_child i = 2 * i + 2
let parent      i = (i - 1) / 2

let get tree i =
  if i < Array.length tree.data then Some tree.data.(i) else None

let is_leaf tree i =
  left_child i >= Array.length tree.data

let left  tree i = get tree (left_child i)
let right tree i = get tree (right_child i)

(* Level-order decomposition *)
let levels tree =
  let n = Array.length tree.data in
  if n = 0 then []
  else begin
    let result = ref [] in
    let start = ref 0 and level_size = ref 1 in
    while !start < n do
      let end_ = min (!start + !level_size) n in
      result := Array.to_list (Array.sub tree.data !start (end_ - !start)) :: !result;
      start := end_;
      level_size := !level_size * 2
    done;
    List.rev !result
  end

(* Depth of the tree *)
let depth tree =
  let n = Array.length tree.data in
  if n = 0 then 0
  else int_of_float (log (float_of_int n) /. log 2.0) + 1

(* Max-heapify in-place using sift-down *)
let heapify tree =
  let arr = Array.copy tree.data in
  let n = Array.length arr in
  let sift_down i =
    let i = ref i in
    let continue_ = ref true in
    while !continue_ do
      let largest = ref !i in
      let l = left_child !i and r = right_child !i in
      if l < n && arr.(l) > arr.(!largest) then largest := l;
      if r < n && arr.(r) > arr.(!largest) then largest := r;
      if !largest = !i then continue_ := false
      else begin
        let tmp = arr.(!i) in
        arr.(!i) <- arr.(!largest);
        arr.(!largest) <- tmp;
        i := !largest
      end
    done
  in
  for i = n / 2 - 1 downto 0 do sift_down i done;
  { data = arr }

let () =
  (*       1
          / \
         2   3
        / \ /
       4  5 6 *)
  let tree = make [|1; 2; 3; 4; 5; 6|] in
  assert (get tree 0 = Some 1);
  assert (left  tree 0 = Some 2);
  assert (right tree 0 = Some 3);
  assert (left  tree 1 = Some 4);
  assert (right tree 1 = Some 5);
  assert (left  tree 2 = Some 6);
  assert (right tree 2 = None);
  assert (is_leaf tree 3);
  assert (not (is_leaf tree 0));

  let tree2 = make [|1;2;3;4;5;6;7|] in
  let lvls = levels tree2 in
  assert (List.length lvls = 3);
  assert (List.nth lvls 0 = [1]);
  assert (List.nth lvls 1 = [2; 3]);
  assert (List.nth lvls 2 = [4; 5; 6; 7]);

  (* Heapify *)
  let tree3 = heapify (make [|3;1;4;1;5;9;2;6|]) in
  assert (tree3.data.(0) = 9);  (* root is max *)
  (* Verify heap property: parent >= all children *)
  let n = Array.length tree3.data in
  for i = 1 to n - 1 do
    assert (tree3.data.(parent i) >= tree3.data.(i))
  done;

  (* Depth *)
  assert (depth (make [|1|]) = 1);
  assert (depth (make [|1;2;3|]) = 2);
  assert (depth (make [|1;2;3;4;5;6;7|]) = 3);
  assert (depth (make [||]) = 0);

  Printf.printf "All flat-tree tests passed.\n"
