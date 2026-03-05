(* OCaml: Semaphore simulation *)

(* OCaml doesn't have a stdlib semaphore.
   For async, use Lwt_pool or implement with Mutex + Condition. *)

type semaphore = {
  count: int ref;
  m: Mutex.t;
  cv: Condition.t;
}

let create_semaphore n =
  { count = ref n; m = Mutex.create (); cv = Condition.create () }

let acquire sem =
  Mutex.lock sem.m;
  while !(sem.count) = 0 do
    Condition.wait sem.cv sem.m
  done;
  decr sem.count;
  Mutex.unlock sem.m

let release sem =
  Mutex.lock sem.m;
  incr sem.count;
  Condition.signal sem.cv;
  Mutex.unlock sem.m

let () =
  let sem = create_semaphore 2 in
  acquire sem;
  Printf.printf "Acquired first permit\n";
  acquire sem;
  Printf.printf "Acquired second permit\n";
  release sem;
  Printf.printf "Released one permit\n"
