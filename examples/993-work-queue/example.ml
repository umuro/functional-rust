(* 993: Work Queue (Thread Pool)
   A fixed pool of worker threads processes tasks from a shared queue.
   Producers enqueue work; workers dequeue and execute it.
   OCaml: Queue + Mutex + Condition (classic work-stealing pattern simplified). *)

type 'a task = unit -> 'a

type 'result work_queue = {
  tasks   : (unit -> unit) Queue.t;
  results : 'result Queue.t;
  mutex   : Mutex.t;
  not_empty : Condition.t;
  not_full  : Condition.t;    (* optional backpressure *)
  done_cond : Condition.t;
  mutable submitted : int;
  mutable completed : int;
  mutable running   : bool;
  workers : Thread.t array;
}

(* Create a pool with n_workers threads *)
let create ?(max_queue=1024) n_workers =
  let q : (unit, unit) work_queue = {
    tasks     = Queue.create ();
    results   = Queue.create ();
    mutex     = Mutex.create ();
    not_empty = Condition.create ();
    not_full  = Condition.create ();
    done_cond = Condition.create ();
    submitted = 0;
    completed = 0;
    running   = true;
    workers   = [||];  (* filled below *)
  } in
  let workers = Array.init n_workers (fun _i ->
    Thread.create (fun () ->
      let continue_ = ref true in
      while !continue_ do
        Mutex.lock q.mutex;
        while Queue.is_empty q.tasks && q.running do
          Condition.wait q.not_empty q.mutex
        done;
        if not q.running && Queue.is_empty q.tasks then begin
          continue_ := false;
          Mutex.unlock q.mutex
        end else if not (Queue.is_empty q.tasks) then begin
          let task = Queue.pop q.tasks in
          Condition.signal q.not_full;
          Mutex.unlock q.mutex;
          task ();
          Mutex.lock q.mutex;
          q.completed <- q.completed + 1;
          if q.completed = q.submitted then
            Condition.broadcast q.done_cond;
          Mutex.unlock q.mutex
        end else
          Mutex.unlock q.mutex
      done
    ) ()
  ) in
  (* Hack: rebuild with workers set — OCaml records are immutable *)
  { q with workers }

(* Submit a task; returns unit (fire and forget) *)
let submit pool f =
  Mutex.lock pool.mutex;
  while Queue.length pool.tasks >= 1024 do
    Condition.wait pool.not_full pool.mutex
  done;
  Queue.push f pool.tasks;
  pool.submitted <- pool.submitted + 1;
  Condition.signal pool.not_empty;
  Mutex.unlock pool.mutex

(* Wait until all submitted tasks have completed *)
let wait_all pool =
  Mutex.lock pool.mutex;
  while pool.completed < pool.submitted do
    Condition.wait pool.done_cond pool.mutex
  done;
  Mutex.unlock pool.mutex

(* Shutdown: no more tasks accepted; drain and stop workers *)
let shutdown pool =
  Mutex.lock pool.mutex;
  pool.running <- false;
  Condition.broadcast pool.not_empty;
  Mutex.unlock pool.mutex;
  Array.iter Thread.join pool.workers

(* Submit with result collection *)
let map_pool pool items f =
  let n = List.length items in
  let results = Array.make n (Obj.magic ()) in
  let mutex = Mutex.create () in
  let done_count = ref 0 in
  let done_cond = Condition.create () in

  List.iteri (fun i x ->
    submit pool (fun () ->
      let v = f x in
      Mutex.lock mutex;
      results.(i) <- v;
      incr done_count;
      if !done_count = n then Condition.broadcast done_cond;
      Mutex.unlock mutex
    )
  ) items;

  Mutex.lock mutex;
  while !done_count < n do Condition.wait done_cond mutex done;
  Mutex.unlock mutex;
  Array.to_list results

let () =
  Printf.printf "=== Work queue with 4 workers ===\n";
  let pool = create 4 in

  let counter = ref 0 in
  let mutex = Mutex.create () in

  for i = 1 to 20 do
    let n = i in
    submit pool (fun () ->
      Thread.delay 0.001;
      Mutex.lock mutex;
      counter := !counter + n;
      Mutex.unlock mutex
    )
  done;

  wait_all pool;
  Printf.printf "sum 1..20 = %d (expected 210)\n" !counter;

  Printf.printf "\n=== Parallel map via pool ===\n";
  let items = List.init 10 (fun i -> i + 1) in
  let squared = map_pool pool items (fun x -> x * x) in
  Printf.printf "squared: [%s]\n"
    (String.concat "; " (List.map string_of_int squared));

  Printf.printf "\n=== Fibonacci in parallel ===\n";
  let fib n =
    let rec f a b n = if n = 0 then a else f b (a+b) (n-1) in f 0 1 n
  in
  let ns = [30; 32; 28; 35; 25] in
  let results = map_pool pool ns fib in
  List.iter2 (fun n v ->
    Printf.printf "  fib(%d) = %d\n" n v
  ) ns results;

  shutdown pool;
  Printf.printf "pool shut down cleanly\n"
