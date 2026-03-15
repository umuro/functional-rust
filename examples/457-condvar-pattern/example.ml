(* 457: Condition Variable Pattern
   OCaml's Condition module works exactly like Rust's Condvar:
   always paired with a Mutex, supporting wait/signal/broadcast. *)

let () =
  let mutex = Mutex.create () in
  let cond  = Condition.create () in
  let ready = ref false in

  (* Notifier domain: sets flag and signals the waiting domain *)
  let notifier = Domain.spawn (fun () ->
    Unix.sleepf 0.005;
    Mutex.lock mutex;
    ready := true;
    Condition.signal cond;
    Mutex.unlock mutex)
  in

  (* Wait until [ready] is true — spurious wakeup safe *)
  Mutex.lock mutex;
  while not !ready do
    Condition.wait cond mutex
  done;
  assert !ready;
  Mutex.unlock mutex;
  Domain.join notifier;
  Printf.printf "condvar notify: ok, ready=%b\n%!" !ready;

  (* Broadcast: wake all waiting domains at once *)
  let n = 4 in
  let mutex2   = Mutex.create () in
  let cond2    = Condition.create () in
  let go       = ref false in
  let arrived  = Atomic.make 0 in

  let waiters = List.init n (fun _ ->
    Domain.spawn (fun () ->
      Mutex.lock mutex2;
      while not !go do
        Condition.wait cond2 mutex2
      done;
      Mutex.unlock mutex2;
      ignore (Atomic.fetch_and_add arrived 1)))
  in

  Unix.sleepf 0.005;
  Mutex.lock mutex2;
  go := true;
  Condition.broadcast cond2;
  Mutex.unlock mutex2;

  List.iter Domain.join waiters;
  assert (Atomic.get arrived = n);
  Printf.printf "condvar broadcast: %d/%d domains woke up\n%!"
    (Atomic.get arrived) n
