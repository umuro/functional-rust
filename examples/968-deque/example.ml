(* 968: Deque (Double-Ended Queue)
   OCaml: two-stack deque (purely functional variant) and a mutable
   circular-buffer variant for O(1) amortized push/pop on both ends.
   The two-stack trick: front list + rear list, re-balance when either runs out. *)

(* --- Purely functional deque (two-list) --- *)
(* Invariant: if one list is non-empty, the other may be empty only
   when |front| >= |rear|. For simplicity we re-balance lazily. *)
module FDeque = struct
  type 'a t = { front : 'a list; rear : 'a list }

  let empty = { front = []; rear = [] }

  let is_empty d = d.front = [] && d.rear = []

  (* Re-balance: split rear in half and reverse onto front *)
  let balance d =
    match d with
    | { front = []; rear } ->
      let n = List.length rear in
      let arr = Array.of_list (List.rev rear) in
      let half = n / 2 in
      let new_front = Array.to_list (Array.sub arr 0 (n - half)) in
      let new_rear  = Array.to_list (Array.sub arr (n - half) half) in
      { front = new_front; rear = List.rev new_rear }
    | d -> d

  let push_front x d = { d with front = x :: d.front }

  let push_back x d = { d with rear = x :: d.rear }

  let pop_front d =
    let d = balance d in
    match d.front with
    | []     -> None
    | x :: rest -> Some (x, { d with front = rest })

  let pop_back d =
    (* Mirror: balance swapping roles *)
    let d' = balance { front = d.rear; rear = d.front } in
    match d'.front with
    | []     -> None
    | x :: rest -> Some (x, { front = d'.rear; rear = rest })

  let peek_front d =
    let d = balance d in
    match d.front with x :: _ -> Some x | [] -> None

  let to_list d = d.front @ List.rev d.rear

  let length d = List.length d.front + List.length d.rear
end

(* --- Mutable circular-buffer deque --- *)
type 'a deque = {
  mutable buf  : 'a array;
  mutable head : int;   (* index of the front element *)
  mutable size : int;
  dummy : 'a;
}

let create_mut ?(capacity=8) dummy =
  { buf = Array.make capacity dummy; head = 0; size = 0; dummy }

let cap d = Array.length d.buf

let grow_mut d =
  let old_cap = cap d in
  let new_buf = Array.make (old_cap * 2) d.dummy in
  for i = 0 to d.size - 1 do
    new_buf.(i) <- d.buf.((d.head + i) mod old_cap)
  done;
  d.buf <- new_buf;
  d.head <- 0

let push_front_mut d x =
  if d.size = cap d then grow_mut d;
  d.head <- (d.head - 1 + cap d) mod cap d;
  d.buf.(d.head) <- x;
  d.size <- d.size + 1

let push_back_mut d x =
  if d.size = cap d then grow_mut d;
  d.buf.((d.head + d.size) mod cap d) <- x;
  d.size <- d.size + 1

let pop_front_mut d =
  if d.size = 0 then None
  else begin
    let x = d.buf.(d.head) in
    d.buf.(d.head) <- d.dummy;
    d.head <- (d.head + 1) mod cap d;
    d.size <- d.size - 1;
    Some x
  end

let pop_back_mut d =
  if d.size = 0 then None
  else begin
    let idx = (d.head + d.size - 1) mod cap d in
    let x = d.buf.(idx) in
    d.buf.(idx) <- d.dummy;
    d.size <- d.size - 1;
    Some x
  end

let () =
  Printf.printf "=== Functional deque ===\n";
  let d = FDeque.empty in
  let d = FDeque.push_back 1 d in
  let d = FDeque.push_back 2 d in
  let d = FDeque.push_back 3 d in
  let d = FDeque.push_front 0 d in
  Printf.printf "contents: [%s]\n"
    (String.concat "; " (List.map string_of_int (FDeque.to_list d)));
  (match FDeque.pop_front d with
   | Some (x, d') ->
     Printf.printf "pop_front = %d, remaining: [%s]\n" x
       (String.concat "; " (List.map string_of_int (FDeque.to_list d')))
   | None -> ());

  Printf.printf "\n=== Mutable circular-buffer deque ===\n";
  let q : int deque = create_mut 0 in
  push_back_mut q 10;
  push_back_mut q 20;
  push_back_mut q 30;
  push_front_mut q 5;
  Printf.printf "size = %d\n" q.size;
  Printf.printf "pop_front = %d\n" (Option.get (pop_front_mut q));
  Printf.printf "pop_back  = %d\n" (Option.get (pop_back_mut q));

  (* Demonstrate growth *)
  let big : int deque = create_mut ~capacity:4 0 in
  for i = 1 to 10 do push_back_mut big i done;
  Printf.printf "\nAfter pushing 1..10: size=%d cap=%d\n" big.size (cap big);
  let all = ref [] in
  while big.size > 0 do all := Option.get (pop_front_mut big) :: !all done;
  Printf.printf "pop order: %s\n"
    (String.concat " " (List.map string_of_int (List.rev !all)))
