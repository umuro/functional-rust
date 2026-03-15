(* 990: Semaphore via Mutex + Condvar *)
(* Counting semaphore: allow at most N concurrent operations *)

type semaphore = {
  mutable count: int;
  max_count: int;
  m: Mutex.t;
  cond: Condition.t;
}

let make_semaphore n = {
  count = n;
  max_count = n;
  m = Mutex.create ();
  cond = Condition.create ();
}

let acquire sem =
  Mutex.lock sem.m;
  while sem.count = 0 do
    Condition.wait sem.cond sem.m
  done;
  sem.count <- sem.count - 1;
  Mutex.unlock sem.m

let release sem =
  Mutex.lock sem.m;
  if sem.count < sem.max_count then begin
    sem.count <- sem.count + 1;
    Condition.signal sem.cond
  end;
  Mutex.unlock sem.m

let with_semaphore sem f =
  acquire sem;
  let result = (try f () with e -> release sem; raise e) in
  release sem;
  result

(* --- Approach 1: Limit concurrent workers to 3 --- *)

let () =
  let sem = make_semaphore 3 in
  let active = ref 0 in
  let max_active = ref 0 in
  let m = Mutex.create () in

  let threads = List.init 10 (fun i ->
    Thread.create (fun () ->
      with_semaphore sem (fun () ->
        Mutex.lock m;
        incr active;
        if !active > !max_active then max_active := !active;
        Mutex.unlock m;

        (* simulate work *)
        Unix.sleepf 0.005;

        Mutex.lock m;
        decr active;
        Mutex.unlock m;

        Printf.printf "worker %d done\n" i
      )
    ) ()
  ) in
  List.iter Thread.join threads;
  assert (!max_active <= 3);
  Printf.printf "Approach 1: max concurrent = %d (≤ 3)\n" !max_active

(* --- Approach 2: Binary semaphore (mutex-like) --- *)

let () =
  let sem = make_semaphore 1 in
  let counter = ref 0 in
  let threads = List.init 5 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 100 do
        with_semaphore sem (fun () -> incr counter)
      done
    ) ()
  ) in
  List.iter Thread.join threads;
  assert (!counter = 500);
  Printf.printf "Approach 2 (binary semaphore): counter = %d\n" !counter

let () = Printf.printf "✓ All tests passed\n"
