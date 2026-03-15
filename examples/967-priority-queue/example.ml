(* 967: Priority Queue (Min-Heap from scratch) *)
(* Binary heap: parent <= children (min-heap) *)

type 'a heap = {
  mutable data: 'a array;
  mutable size: int;
  compare: 'a -> 'a -> int;
}

let create ?(capacity=16) compare =
  { data = Array.make capacity (Obj.magic 0);
    size = 0;
    compare }

let swap h i j =
  let tmp = h.data.(i) in
  h.data.(i) <- h.data.(j);
  h.data.(j) <- tmp

let sift_up h i =
  let i = ref i in
  while !i > 0 do
    let parent = (!i - 1) / 2 in
    if h.compare h.data.(!i) h.data.(parent) < 0 then begin
      swap h !i parent;
      i := parent
    end else
      i := 0  (* break *)
  done

let sift_down h i =
  let i = ref i in
  let continue_ = ref true in
  while !continue_ do
    let left = 2 * !i + 1 in
    let right = 2 * !i + 2 in
    let smallest = ref !i in
    if left < h.size && h.compare h.data.(left) h.data.(!smallest) < 0 then
      smallest := left;
    if right < h.size && h.compare h.data.(right) h.data.(!smallest) < 0 then
      smallest := right;
    if !smallest <> !i then begin
      swap h !i !smallest;
      i := !smallest
    end else
      continue_ := false
  done

let push h x =
  if h.size = Array.length h.data then begin
    let new_data = Array.make (h.size * 2) (Obj.magic 0) in
    Array.blit h.data 0 new_data 0 h.size;
    h.data <- new_data
  end;
  h.data.(h.size) <- x;
  h.size <- h.size + 1;
  sift_up h (h.size - 1)

let peek h =
  if h.size = 0 then None
  else Some h.data.(0)

let pop h =
  if h.size = 0 then None
  else begin
    let top = h.data.(0) in
    h.size <- h.size - 1;
    if h.size > 0 then begin
      h.data.(0) <- h.data.(h.size);
      sift_down h 0
    end;
    Some top
  end

let size h = h.size

let () =
  let h = create compare in
  push h 5;
  push h 3;
  push h 8;
  push h 1;
  push h 9;
  push h 2;

  assert (peek h = Some 1);
  assert (size h = 6);

  assert (pop h = Some 1);
  assert (pop h = Some 2);
  assert (pop h = Some 3);
  assert (pop h = Some 5);
  assert (pop h = Some 8);
  assert (pop h = Some 9);
  assert (pop h = None);

  (* Heap sort test *)
  let h2 = create compare in
  List.iter (push h2) [4; 7; 2; 1; 8; 3; 6; 5];
  let sorted = ref [] in
  while size h2 > 0 do
    sorted := (pop h2 |> Option.get) :: !sorted
  done;
  assert (List.rev !sorted = [1;2;3;4;5;6;7;8]);

  Printf.printf "✓ All tests passed\n"
