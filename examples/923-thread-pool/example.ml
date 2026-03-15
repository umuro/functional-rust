(* 923: Thread Pool — fixed-size worker pool with a job queue

   OCaml: Thread + Mutex + Condition for a classic bounded thread pool.
   Workers block on a shared queue; the main thread enqueues jobs;
   a `finish` call waits for all work to complete. *)

(* ── Job queue ───────────────────────────────────────────────────────────── *)

type 'a job =
  | Work of (unit -> 'a)
  | Shutdown

type 'a pool = {
  workers     : Thread.t list;
  queue       : 'a job Queue.t;
  results     : (int * 'a) list ref;
  q_mutex     : Mutex.t;
  q_cond      : Condition.t;
  r_mutex     : Mutex.t;
  mutable next_id : int;
}

(* ── Pool construction ────────────────────────────────────────────────────── *)

let make_pool size =
  let pool = {
    workers  = [];
    queue    = Queue.create ();
    results  = ref [];
    q_mutex  = Mutex.create ();
    q_cond   = Condition.create ();
    r_mutex  = Mutex.create ();
    next_id  = 0;
  } in
  let worker_loop () =
    let running = ref true in
    while !running do
      Mutex.lock pool.q_mutex;
      while Queue.is_empty pool.queue do
        Condition.wait pool.q_cond pool.q_mutex
      done;
      let job = Queue.pop pool.queue in
      Mutex.unlock pool.q_mutex;
      match job with
      | Shutdown -> running := false
      | Work f ->
        let result = f () in
        Mutex.lock pool.r_mutex;
        pool.results := result :: !(pool.results);
        Mutex.unlock pool.r_mutex
    done
  in
  let workers = List.init size (fun _ -> Thread.create worker_loop ()) in
  { pool with workers }

(* Submit a job — returns unit (results collected separately) *)
let submit pool f =
  Mutex.lock pool.q_mutex;
  Queue.push (Work f) pool.queue;
  Condition.signal pool.q_cond;
  Mutex.unlock pool.q_mutex

(* Shut down: send Shutdown to each worker, then join *)
let shutdown pool =
  List.iter (fun _ ->
    Mutex.lock pool.q_mutex;
    Queue.push Shutdown pool.queue;
    Condition.signal pool.q_cond;
    Mutex.unlock pool.q_mutex
  ) pool.workers;
  List.iter Thread.join pool.workers

(* ── Parallel map using pool ─────────────────────────────────────────────── *)

(* For ordered results we record (index, value) pairs *)
let parallel_map ~workers items f =
  let n = List.length items in
  let results = Array.make n None in
  let pending = ref n in
  let done_mutex = Mutex.create () in
  let done_cond  = Condition.create () in
  let pool = make_pool workers in
  List.iteri (fun i x ->
    submit pool (fun () ->
      let v = f x in
      results.(i) <- Some v;
      Mutex.lock done_mutex;
      decr pending;
      if !pending = 0 then Condition.signal done_cond;
      Mutex.unlock done_mutex
    )
  ) items;
  (* Wait for all tasks *)
  Mutex.lock done_mutex;
  while !pending > 0 do Condition.wait done_cond done_mutex done;
  Mutex.unlock done_mutex;
  shutdown pool;
  Array.to_list (Array.map Option.get results)

let () =
  (* parallel map preserves order *)
  let result = parallel_map ~workers:4 [1; 2; 3; 4; 5] (fun x -> x * 2) in
  assert (result = [2; 4; 6; 8; 10]);

  (* single worker = sequential *)
  let result2 = parallel_map ~workers:1 [1; 2; 3] (fun x -> x + 10) in
  assert (result2 = [11; 12; 13]);

  (* strings *)
  let result3 = parallel_map ~workers:2 ["hello"; "world"; "ocaml"]
    (fun s -> String.uppercase_ascii s) in
  assert (result3 = ["HELLO"; "WORLD"; "OCAML"]);

  print_endline "923-thread-pool: all tests passed"
