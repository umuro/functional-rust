(* 353: VecDeque / Double-ended Queue
   OCaml's Queue module is singly-ended; for a true deque we use
   the two-list functional deque or Doubly_linked from Core.
   Here we implement a simple mutable deque with an array-based approach,
   and demonstrate the same sliding-window and rotate patterns. *)

(* Simple functional deque using two lists (front reversed, back forward) *)
type 'a deque = { front: 'a list; back: 'a list }

let empty_deque = { front = []; back = [] }

let push_back  d x = { d with back  = x :: d.back }
let push_front d x = { d with front = x :: d.front }

let pop_front d =
  match d.front with
  | x :: front -> Some (x, { d with front })
  | [] ->
    match List.rev d.back with
    | []         -> None
    | x :: front -> Some (x, { front; back = [] })

let pop_back d =
  match d.back with
  | x :: back -> Some (x, { d with back })
  | [] ->
    match List.rev d.front with
    | []        -> None
    | x :: back -> Some (x, { back; front = [] })

let deque_length d = List.length d.front + List.length d.back

let deque_to_list d = d.front @ List.rev d.back

(* Sliding window using a deque *)
let sliding_window data window_size =
  let result = ref [] in
  let window = ref empty_deque in
  List.iter (fun item ->
    window := push_back !window item;
    if deque_length !window > window_size then begin
      match pop_front !window with
      | Some (_, w) -> window := w
      | None -> ()
    end;
    if deque_length !window = window_size then
      result := deque_to_list !window :: !result
  ) data;
  List.rev !result

(* Rotate left by n positions *)
let rotate_left items n =
  let len = List.length items in
  if len = 0 then items
  else
    let n = n mod len in
    let a, b = List.filteri (fun i _ -> i >= n) items,
               List.filteri (fun i _ -> i < n)  items in
    a @ b

let () =
  (* Sliding window *)
  let windows = sliding_window [1;2;3;4;5] 3 in
  assert (windows = [[1;2;3];[2;3;4];[3;4;5]]);
  Printf.printf "sliding windows: %s\n%!"
    (windows |> List.map (fun w ->
       "[" ^ (w |> List.map string_of_int |> String.concat ",") ^ "]")
     |> String.concat " ");

  (* Rotate left *)
  let r = rotate_left [1;2;3;4;5] 2 in
  assert (r = [3;4;5;1;2]);
  Printf.printf "rotate_left 2: %s\n%!"
    (r |> List.map string_of_int |> String.concat ", ");

  (* Deque both-ends push/pop *)
  let d = empty_deque in
  let d = push_back  d 2 in
  let d = push_front d 1 in
  let d = push_back  d 3 in
  (match pop_front d with
   | Some (v, d') ->
     assert (v = 1);
     (match pop_back d' with
      | Some (v2, _) -> assert (v2 = 3);
        Printf.printf "deque: front=%d back=%d\n%!" v v2
      | None -> assert false)
   | None -> assert false)
