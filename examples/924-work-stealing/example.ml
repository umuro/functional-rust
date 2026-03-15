(* 924: Work Stealing — each worker has its own deque; idle workers steal
   from the back of busy workers' deques.

   OCaml doesn't have a stdlib work-stealing deque, so we model the concept
   with per-thread queues (deques implemented as doubly-indexed arrays) and
   a simple steal: when a worker's local queue is empty it tries to take a
   task from another worker's queue. *)

(* ── Per-worker deque (simple circular buffer approximation via list ref) ─── *)

type 'a worker_state = {
  id      : int;
  local   : 'a Queue.t;
  mutex   : Mutex.t;
}

let make_worker id = {
  id;
  local = Queue.create ();
  mutex = Mutex.create ();
}

(* Push to local queue *)
let push_local w task =
  Mutex.lock w.mutex;
  Queue.push task w.local;
  Mutex.unlock w.mutex

(* Pop from local queue (lifo-like: take from front) *)
let pop_local w =
  Mutex.lock w.mutex;
  let r = if Queue.is_empty w.local then None else Some (Queue.pop w.local) in
  Mutex.unlock w.mutex;
  r

(* Steal from another worker's queue (take from back = Queue.pop for us) *)
let steal_from w =
  Mutex.lock w.mutex;
  let r =
    if Queue.length w.local > 1 then begin
      (* Steal the "oldest" task — simplification: pop last element *)
      (* Queue doesn't expose pop_last, so we transfer to a temp *)
      let items = Queue.fold (fun acc x -> x :: acc) [] w.local in
      Queue.clear w.local;
      match items with
      | [] -> None
      | stolen :: rest ->
        List.iter (fun x -> Queue.push x w.local) (List.rev rest);
        Some stolen
    end else None
  in
  Mutex.unlock w.mutex;
  r

(* ── Work-stealing pool ──────────────────────────────────────────────────── *)

let work_stealing_map ~workers items f =
  let n = List.length items in
  if n = 0 then []
  else begin
    let results = Array.make n None in
    let ws = Array.init workers make_worker in
    (* Distribute tasks round-robin *)
    List.iteri (fun i task ->
      push_local ws.(i mod workers) (i, task)
    ) items;
    let remaining = ref n in
    let done_mutex = Mutex.create () in
    let done_cond  = Condition.create () in
    let threads = Array.init workers (fun id ->
      Thread.create (fun () ->
        let keep_going = ref true in
        while !keep_going do
          (* Try own queue first *)
          let task = match pop_local ws.(id) with
            | Some t -> Some t
            | None ->
              (* Try stealing from other workers *)
              let stolen = ref None in
              Array.iteri (fun other_id other_w ->
                if !stolen = None && other_id <> id then
                  stolen := steal_from other_w
              ) ws;
              !stolen
          in
          match task with
          | None -> keep_going := false  (* no work available, exit *)
          | Some (idx, item) ->
            results.(idx) <- Some (f item);
            Mutex.lock done_mutex;
            decr remaining;
            if !remaining = 0 then Condition.signal done_cond;
            Mutex.unlock done_mutex
        done
      ) ()
    ) in
    (* Wait for all results *)
    Mutex.lock done_mutex;
    while !remaining > 0 do Condition.wait done_cond done_mutex done;
    Mutex.unlock done_mutex;
    Array.iter Thread.join threads;
    Array.to_list (Array.map Option.get results)
  end

let () =
  (* work_stealing_map preserves order *)
  let result = work_stealing_map ~workers:4 [1; 2; 3; 4; 5] (fun x -> x * 2) in
  assert (result = [2; 4; 6; 8; 10]);

  let result2 = work_stealing_map ~workers:2 [1; 2; 3] (fun x -> x + 10) in
  assert (result2 = [11; 12; 13]);

  (* empty input *)
  let result3 = work_stealing_map ~workers:4 [] (fun x -> x) in
  assert (result3 = []);

  (* larger workload *)
  let big = List.init 100 (fun i -> i + 1) in
  let expected = List.init 100 (fun i -> (i + 1) * (i + 1)) in
  let result4 = work_stealing_map ~workers:4 big (fun x -> x * x) in
  assert (result4 = expected);

  print_endline "924-work-stealing: all tests passed"
