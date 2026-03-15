(* 1032: VecDeque Rotation — Efficient Front/Back Operations
   OCaml has no built-in VecDeque; we use a functional deque (two-list representation)
   or a mutable Queue. For rotation, we show both.
   The two-list deque achieves amortized O(1) push/pop at both ends. *)

(* Functional double-ended queue: (front_list, rear_list) *)
(* Pop from front by taking from front_list; if empty, reverse rear_list *)
type 'a deque = { front: 'a list; rear: 'a list }

let empty = { front = []; rear = [] }

let push_back  dq x = { dq with rear  = x :: dq.rear  }
let push_front dq x = { dq with front = x :: dq.front }

let pop_front dq =
  match dq.front with
  | x :: rest -> Some (x, { dq with front = rest })
  | [] ->
    (match List.rev dq.rear with
     | [] -> None
     | x :: rest -> Some (x, { front = rest; rear = [] }))

let pop_back dq =
  match dq.rear with
  | x :: rest -> Some (x, { dq with rear = rest })
  | [] ->
    (match List.rev dq.front with
     | [] -> None
     | x :: rest -> Some (x, { rear = rest; front = [] }))

let of_list lst = List.fold_left push_back empty lst

let to_list dq = dq.front @ List.rev dq.rear

(* Rotate left by n: move first n elements to the back *)
let rotate_left n dq =
  let lst = to_list dq in
  let len = List.length lst in
  if len = 0 then dq
  else
    let n' = n mod len in
    let (head, tail) = (List.filteri (fun i _ -> i < n') lst,
                        List.filteri (fun i _ -> i >= n') lst) in
    of_list (tail @ head)

let rotate_right n dq =
  let lst = to_list dq in
  let len = List.length lst in
  if len = 0 then dq
  else
    let n' = n mod len in
    rotate_left (len - n') dq

(* Basic front/back operations *)
let basic_deque () =
  let dq = empty
    |> fun d -> push_back d 1
    |> fun d -> push_back d 2
    |> fun d -> push_back d 3
    |> fun d -> push_front d 0
  in
  let (Some (v0, dq)) = pop_front dq in assert (v0 = 0);
  let (Some (v1, dq)) = pop_front dq in assert (v1 = 1);
  let (Some (v3, dq)) = pop_back  dq in assert (v3 = 3);
  let (Some (v2, dq)) = pop_back  dq in assert (v2 = 2);
  assert (to_list dq = [])

let rotation () =
  let dq = of_list [1; 2; 3; 4; 5] in
  let dq2 = rotate_left 2 dq in
  assert (to_list dq2 = [3; 4; 5; 1; 2]);
  let dq3 = rotate_right 2 dq2 in
  assert (to_list dq3 = [1; 2; 3; 4; 5])

(* Sliding window using a list-based deque *)
let sliding_window data window_size =
  let rec loop window sums rest =
    match rest with
    | [] -> List.rev sums
    | v :: tl ->
      let win = window @ [v] in
      let win' = if List.length win > window_size then List.tl win else win in
      let sums' = if List.length win' = window_size
                  then (List.fold_left (+) 0 win') :: sums
                  else sums
      in
      loop win' sums' tl
  in
  loop [] [] data

let () =
  basic_deque ();
  rotation ();

  let sums = sliding_window [1; 2; 3; 4; 5; 6; 7] 3 in
  assert (sums = [6; 9; 12; 15; 18]);

  (* Conversion: list -> deque -> list *)
  let dq = of_list [1; 2; 3; 4; 5] in
  let dq' = push_front dq 0 in
  assert (to_list dq' = [0; 1; 2; 3; 4; 5]);

  (* Indexed access *)
  let dq2 = of_list [10; 20; 30] in
  let lst = to_list dq2 in
  assert (List.nth lst 0 = 10);
  assert (List.nth lst 1 = 20);
  assert (List.nth lst 2 = 30);

  Printf.printf "VecDeque (functional deque) tests passed\n"
