(* 990: Semaphore
   Counting semaphore: limits the number of concurrent accesses to a resource.
   Binary semaphore (max=1) acts as a mutex.
   OCaml: Mutex + Condition variable (no semaphore in stdlib). *)

type semaphore = {
  mutable count : int;
  max   : int;
  mutex : Mutex.t;
  cond  : Condition.t;
}

(* Create a semaphore with initial permits *)
let create ?(max=max_int) init =
  assert (init >= 0 && init <= max);
  { count = init; max; mutex = Mutex.create (); cond = Condition.create () }

(* Acquire (P / down / wait): block until a permit is available *)
let acquire sem =
  Mutex.lock sem.mutex;
  while sem.count = 0 do
    Condition.wait sem.cond sem.mutex
  done;
  sem.count <- sem.count - 1;
  Mutex.unlock sem.mutex

(* Try-acquire: non-blocking; returns true if a permit was taken *)
let try_acquire sem =
  Mutex.lock sem.mutex;
  let r = if sem.count > 0 then (sem.count <- sem.count - 1; true) else false in
  Mutex.unlock sem.mutex;
  r

(* Release (V / up / signal): return a permit *)
let release sem =
  Mutex.lock sem.mutex;
  if sem.count < sem.max then begin
    sem.count <- sem.count + 1;
    Condition.signal sem.cond
  end;
  Mutex.unlock sem.mutex

(* RAII: acquire, run f, release even on exception *)
let with_semaphore sem f =
  acquire sem;
  match f () with
  | v -> release sem; v
  | exception e -> release sem; raise e

let permits sem =
  Mutex.lock sem.mutex;
  let c = sem.count in
  Mutex.unlock sem.mutex;
  c

let () =
  Printf.printf "=== Connection pool semaphore (max 3 concurrent) ===\n";
  let pool = create ~max:3 3 in
  let active = ref 0 and max_active = ref 0 in
  let mutex = Mutex.create () in

  let threads = List.init 8 (fun i ->
    Thread.create (fun () ->
      with_semaphore pool (fun () ->
        Mutex.lock mutex;
        active := !active + 1;
        if !active > !max_active then max_active := !active;
        Mutex.unlock mutex;

        Printf.printf "  worker %d: using connection (active=%d)\n%!" i !active;
        Thread.delay 0.01;

        Mutex.lock mutex;
        active := !active - 1;
        Mutex.unlock mutex
      )
    ) ()
  ) in
  List.iter Thread.join threads;
  Printf.printf "max concurrent = %d (limit = 3)\n" !max_active;
  Printf.printf "permits after = %d (back to 3)\n" (permits pool);

  Printf.printf "\n=== Binary semaphore as mutex ===\n";
  let bin_sem = create ~max:1 1 in
  let counter = ref 0 in
  let workers = List.init 5 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 200 do
        with_semaphore bin_sem (fun () ->
          counter := !counter + 1)
      done
    ) ()
  ) in
  List.iter Thread.join workers;
  Printf.printf "counter = %d (expected 1000)\n" !counter;

  Printf.printf "\n=== Semaphore for producer-consumer throttling ===\n";
  (* Producer makes items; consumer must signal when it finishes each one *)
  let items_ready = create 0 in  (* 0 permits initially *)
  let results = ref [] and res_mutex = Mutex.create () in

  let producer = Thread.create (fun () ->
    for i = 1 to 5 do
      Thread.delay 0.005;
      Printf.printf "  produced %d\n%!" i;
      release items_ready  (* signal one item available *)
    done
  ) () in

  let consumer = Thread.create (fun () ->
    for _ = 1 to 5 do
      acquire items_ready;  (* wait for an item *)
      let v = permits items_ready in
      Mutex.lock res_mutex;
      results := v :: !results;
      Mutex.unlock res_mutex;
      Printf.printf "  consumed (permits remaining=%d)\n%!" v
    done
  ) () in

  Thread.join producer;
  Thread.join consumer;
  Printf.printf "processed all 5 items: %b\n" (List.length !results = 5)
