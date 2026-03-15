(* 987: Read-Write Lock Pattern *)
(* OCaml: Simulated RwLock using reader count + writer mutex *)

(* --- RwLock simulation: multiple readers OR one writer --- *)

type 'a rwlock = {
  mutable data: 'a;
  mutable readers: int;
  m: Mutex.t;
  can_write: Condition.t;
  can_read: Condition.t;
  mutable writer_waiting: bool;
}

let make_rwlock v = {
  data = v;
  readers = 0;
  m = Mutex.create ();
  can_write = Condition.create ();
  can_read = Condition.create ();
  writer_waiting = false;
}

let read_lock rw =
  Mutex.lock rw.m;
  while rw.writer_waiting do
    Condition.wait rw.can_read rw.m
  done;
  rw.readers <- rw.readers + 1;
  Mutex.unlock rw.m

let read_unlock rw =
  Mutex.lock rw.m;
  rw.readers <- rw.readers - 1;
  if rw.readers = 0 then Condition.signal rw.can_write;
  Mutex.unlock rw.m

let write_lock rw =
  Mutex.lock rw.m;
  rw.writer_waiting <- true;
  while rw.readers > 0 do
    Condition.wait rw.can_write rw.m
  done

let write_unlock rw =
  rw.writer_waiting <- false;
  Condition.broadcast rw.can_read;
  Mutex.unlock rw.m

let with_read rw f =
  read_lock rw;
  let result = (try f rw.data with e -> read_unlock rw; raise e) in
  read_unlock rw;
  result

let with_write rw f =
  write_lock rw;
  (try f rw with e -> write_unlock rw; raise e);
  write_unlock rw

(* --- Approach 1: Multiple readers, no conflict --- *)

let () =
  let rw = make_rwlock 42 in
  (* Multiple concurrent readers *)
  let threads = List.init 5 (fun _ ->
    Thread.create (fun () ->
      let v = with_read rw (fun x -> x) in
      assert (v = 42)
    ) ()
  ) in
  List.iter Thread.join threads;
  Printf.printf "Approach 1 (multiple readers): all read %d\n" 42

(* --- Approach 2: Writer updates, readers see new value --- *)

let () =
  let rw = make_rwlock 0 in

  let writer = Thread.create (fun () ->
    with_write rw (fun rw -> rw.data <- 100)
  ) () in
  Thread.join writer;

  let v = with_read rw (fun x -> x) in
  assert (v = 100);
  Printf.printf "Approach 2 (writer then reader): %d\n" v

let () = Printf.printf "✓ All tests passed\n"
