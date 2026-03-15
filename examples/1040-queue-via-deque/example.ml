(* 1040: Queue Using a Functional Deque / Queue Module
   OCaml's standard Queue module provides O(1) amortized FIFO operations.
   Also shows a purely functional two-list queue. *)

(* Approach 1: OCaml Queue module (imperative, O(1) amortized) *)
let queue_module_demo () =
  let q = Queue.create () in
  Queue.add 1 q;
  Queue.add 2 q;
  Queue.add 3 q;
  assert (Queue.length q = 3);
  assert (Queue.peek q = 1);
  assert (Queue.pop q = 1);
  assert (Queue.pop q = 2);
  assert (Queue.pop q = 3);
  assert (Queue.is_empty q)

(* Approach 2: Purely functional two-list queue
   Enqueue to rear list, dequeue from front list.
   When front is empty, reverse rear into front. *)
type 'a fqueue = { front : 'a list; rear : 'a list }

let empty_q = { front = []; rear = [] }

let enqueue x q = { q with rear = x :: q.rear }

let dequeue q =
  match q.front with
  | x :: front -> (Some x, { q with front })
  | [] ->
    match List.rev q.rear with
    | []     -> (None, empty_q)
    | x :: f -> (Some x, { front = f; rear = [] })

let peek_q q =
  match q.front with
  | x :: _ -> Some x
  | [] ->
    match List.rev q.rear with
    | []     -> None
    | x :: _ -> Some x

let is_empty_q q = q.front = [] && q.rear = []

let fqueue_demo () =
  let q = enqueue 3 (enqueue 2 (enqueue 1 empty_q)) in
  assert (peek_q q = Some 1);
  let (v, q) = dequeue q in assert (v = Some 1);
  let (v, q) = dequeue q in assert (v = Some 2);
  let (v, q) = dequeue q in assert (v = Some 3);
  let (v, _) = dequeue q in assert (v = None)

(* BFS with level tracking using Queue *)
let bfs_levels adjacency start =
  let n = Array.length adjacency in
  let visited = Array.make n false in
  let q = Queue.create () in
  let levels = ref [] in
  visited.(start) <- true;
  Queue.add (start, 0) q;
  while not (Queue.is_empty q) do
    let (node, level) = Queue.pop q in
    (* Extend levels list if needed *)
    while List.length !levels <= level do
      levels := !levels @ [[]]
    done;
    (* Append node to its level *)
    levels := List.mapi (fun i lvl -> if i = level then lvl @ [node] else lvl) !levels;
    Array.iter (fun nb ->
      if not visited.(nb) then begin
        visited.(nb) <- true;
        Queue.add (nb, level + 1) q
      end) adjacency.(node)
  done;
  !levels

let () =
  queue_module_demo ();
  fqueue_demo ();

  let adj = [| [|1;2|]; [|3|]; [|3|]; [||] |] in
  let levels = bfs_levels adj 0 in
  assert (List.nth levels 0 = [0]);
  assert (List.nth levels 1 = [1; 2]);
  assert (List.nth levels 2 = [3]);

  Printf.printf "All queue tests passed.\n"
