(* 967: Priority Queue (Binary Heap)
   OCaml stdlib provides no heap, so we build a min-heap on a dynamic array.
   A max-heap is also shown by negating the comparator.
   The heap property: parent <= children (for min-heap). *)

type 'a heap = {
  mutable data : 'a array;
  mutable size : int;
  cmp  : 'a -> 'a -> int;  (* negative = first arg has higher priority *)
  dummy : 'a;              (* fill value for empty slots *)
}

let create ?(capacity=16) cmp dummy =
  { data = Array.make capacity dummy; size = 0; cmp; dummy }

let min_heap dummy = create compare dummy
let max_heap dummy = create (fun a b -> compare b a) dummy

let swap h i j =
  let tmp = h.data.(i) in
  h.data.(i) <- h.data.(j);
  h.data.(j) <- tmp

(* Double capacity when full *)
let grow h =
  let cap = Array.length h.data in
  let new_data = Array.make (cap * 2) h.dummy in
  Array.blit h.data 0 new_data 0 h.size;
  h.data <- new_data

let sift_up h i =
  let i = ref i in
  while !i > 0 && h.cmp h.data.((!i - 1) / 2) h.data.(!i) > 0 do
    let parent = (!i - 1) / 2 in
    swap h !i parent;
    i := parent
  done

let sift_down h i =
  let i = ref i in
  let continue_ = ref true in
  while !continue_ do
    let left  = 2 * !i + 1 in
    let right = 2 * !i + 2 in
    let best  = ref !i in
    if left  < h.size && h.cmp h.data.(left)  h.data.(!best) < 0 then best := left;
    if right < h.size && h.cmp h.data.(right) h.data.(!best) < 0 then best := right;
    if !best <> !i then begin swap h !i !best; i := !best end
    else continue_ := false
  done

let push h x =
  if h.size >= Array.length h.data then grow h;
  h.data.(h.size) <- x;
  sift_up h h.size;
  h.size <- h.size + 1

let peek h =
  if h.size = 0 then None else Some h.data.(0)

let pop h =
  if h.size = 0 then None
  else begin
    let top = h.data.(0) in
    h.size <- h.size - 1;
    h.data.(0) <- h.data.(h.size);
    h.data.(h.size) <- h.dummy;
    if h.size > 0 then sift_down h 0;
    Some top
  end

let is_empty h = h.size = 0
let size h = h.size

(* Build from array in O(n) via heapify *)
let of_array cmp dummy arr =
  let h = { data = Array.copy arr; size = Array.length arr; cmp; dummy } in
  (* sift down all internal nodes from bottom up *)
  for i = (h.size / 2) - 1 downto 0 do
    sift_down h i
  done;
  h

(* Heap sort: repeatedly pop from a min-heap *)
let heap_sort arr =
  let h = of_array compare arr.(0) arr in
  Array.init (Array.length arr) (fun _ -> Option.get (pop h))

let () =
  (* Min-heap demo *)
  let h : int heap = min_heap 0 in
  List.iter (push h) [5; 3; 8; 1; 4; 2; 7; 6];
  Printf.printf "Min-heap pop order: ";
  while not (is_empty h) do
    Printf.printf "%d " (Option.get (pop h))
  done;
  print_newline ();

  (* Max-heap demo *)
  let mh : int heap = max_heap 0 in
  List.iter (push mh) [5; 3; 8; 1; 4; 2; 7; 6];
  Printf.printf "Max-heap pop order: ";
  while not (is_empty mh) do
    Printf.printf "%d " (Option.get (pop mh))
  done;
  print_newline ();

  (* Priority queue with (priority, task) pairs *)
  Printf.printf "\nTask scheduling (min priority first):\n";
  let tasks : (int * string) heap = create (fun (p1,_) (p2,_) -> compare p1 p2) (0,"") in
  push tasks (3, "low-priority task");
  push tasks (1, "critical task");
  push tasks (2, "normal task");
  while not (is_empty tasks) do
    let (p, name) = Option.get (pop tasks) in
    Printf.printf "  priority=%d  %s\n" p name
  done;

  (* Heap sort *)
  let arr = [|9; 3; 7; 1; 5; 8; 2; 6; 4|] in
  let sorted = heap_sort arr in
  Printf.printf "\nHeap sort: ";
  Array.iter (Printf.printf "%d ") sorted;
  print_newline ()
