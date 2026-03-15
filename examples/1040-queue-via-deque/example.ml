(* 1040: Queue Using VecDeque *)
(* OCaml has a mutable Queue module in stdlib *)

(* Approach 1: OCaml's Queue module *)
let stdlib_queue () =
  let q = Queue.create () in
  Queue.push 1 q;
  Queue.push 2 q;
  Queue.push 3 q;
  assert (Queue.length q = 3);
  assert (Queue.peek q = 1);
  assert (Queue.pop q = 1);
  assert (Queue.pop q = 2);
  assert (Queue.pop q = 3);
  assert (Queue.is_empty q)

(* Approach 2: Functional queue using two lists *)
type 'a fqueue = { inbox: 'a list; outbox: 'a list }

let empty_q = { inbox = []; outbox = [] }

let enqueue x q = { q with inbox = x :: q.inbox }

let dequeue q =
  match q.outbox with
  | x :: rest -> Some (x, { q with outbox = rest })
  | [] ->
    match List.rev q.inbox with
    | [] -> None
    | x :: rest -> Some (x, { inbox = []; outbox = rest })

let fqueue_size q = List.length q.inbox + List.length q.outbox

let functional_queue () =
  let q = empty_q in
  let q = enqueue 1 q in
  let q = enqueue 2 q in
  let q = enqueue 3 q in
  assert (fqueue_size q = 3);
  let (v, q) = Option.get (dequeue q) in
  assert (v = 1);
  let (v, q) = Option.get (dequeue q) in
  assert (v = 2);
  let (v, _q) = Option.get (dequeue q) in
  assert (v = 3)

(* Approach 3: BFS using Queue *)
let bfs_levels adjacency start =
  let n = Array.length adjacency in
  let visited = Array.make n false in
  let q = Queue.create () in
  Queue.push (start, 0) q;
  visited.(start) <- true;
  let levels = Hashtbl.create 16 in
  while not (Queue.is_empty q) do
    let (node, level) = Queue.pop q in
    let current = try Hashtbl.find levels level with Not_found -> [] in
    Hashtbl.replace levels level (current @ [node]);
    List.iter (fun neighbor ->
      if not visited.(neighbor) then begin
        visited.(neighbor) <- true;
        Queue.push (neighbor, level + 1) q
      end
    ) adjacency.(node)
  done;
  levels

let bfs_test () =
  let adj = [| [1; 2]; [3]; [3]; [] |] in
  let levels = bfs_levels adj 0 in
  assert (Hashtbl.find levels 0 = [0]);
  assert (Hashtbl.find levels 1 = [1; 2]);
  assert (Hashtbl.find levels 2 = [3])

let () =
  stdlib_queue ();
  functional_queue ();
  bfs_test ();
  Printf.printf "✓ All tests passed\n"
