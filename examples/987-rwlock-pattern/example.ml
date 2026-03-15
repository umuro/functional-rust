(* 987: RwLock Pattern
   Readers-writer lock: multiple concurrent readers OR one exclusive writer.
   OCaml's stdlib has no RwLock, so we build one with Mutex + Condition.
   Semantics: any number of readers can hold the lock simultaneously;
   a writer waits until all readers release, then gets exclusive access. *)

type rwlock = {
  mutex    : Mutex.t;
  cond     : Condition.t;
  mutable readers : int;
  mutable writer  : bool;
  mutable waiting_writers : int;
}

let make_rwlock () =
  { mutex = Mutex.create (); cond = Condition.create ();
    readers = 0; writer = false; waiting_writers = 0 }

let read_lock rw =
  Mutex.lock rw.mutex;
  (* Wait if there is an active writer or writers waiting (writer preference) *)
  while rw.writer || rw.waiting_writers > 0 do
    Condition.wait rw.cond rw.mutex
  done;
  rw.readers <- rw.readers + 1;
  Mutex.unlock rw.mutex

let read_unlock rw =
  Mutex.lock rw.mutex;
  rw.readers <- rw.readers - 1;
  if rw.readers = 0 then Condition.broadcast rw.cond;
  Mutex.unlock rw.mutex

let write_lock rw =
  Mutex.lock rw.mutex;
  rw.waiting_writers <- rw.waiting_writers + 1;
  while rw.writer || rw.readers > 0 do
    Condition.wait rw.cond rw.mutex
  done;
  rw.waiting_writers <- rw.waiting_writers - 1;
  rw.writer <- true;
  Mutex.unlock rw.mutex

let write_unlock rw =
  Mutex.lock rw.mutex;
  rw.writer <- false;
  Condition.broadcast rw.cond;
  Mutex.unlock rw.mutex

(* RAII helpers *)
let with_read rw f =
  read_lock rw;
  match f () with
  | v -> read_unlock rw; v
  | exception e -> read_unlock rw; raise e

let with_write rw f =
  write_lock rw;
  match f () with
  | v -> write_unlock rw; v
  | exception e -> write_unlock rw; raise e

(* --- Protected shared data using an rwlock --- *)
type 'a rwprotected = {
  mutable data : 'a;
  lock : rwlock;
}

let make_rw v = { data = v; lock = make_rwlock () }
let read_rw p = with_read p.lock (fun () -> p.data)
let write_rw p f = with_write p.lock (fun () -> p.data <- f p.data)

let () =
  Printf.printf "=== RwLock: concurrent readers ===\n";
  let db : (string * int) list rwprotected = make_rw [] in

  (* Writer: initialise the database *)
  write_rw db (fun _ -> [("alice", 100); ("bob", 200); ("carol", 150)]);

  (* Spawn 5 concurrent readers *)
  let reader_log = make_rw [] in
  let readers = List.init 5 (fun i ->
    Thread.create (fun () ->
      let data = read_rw db in
      let total = List.fold_left (fun acc (_, v) -> acc + v) 0 data in
      write_rw reader_log (fun log -> (i, total) :: log)
    ) ()
  ) in
  List.iter Thread.join readers;
  let log = read_rw reader_log in
  Printf.printf "5 readers all saw total=%d: %b\n"
    450
    (List.for_all (fun (_, t) -> t = 450) log);

  (* Writer updates while readers may be active *)
  Printf.printf "\n=== RwLock: writer waits for readers ===\n";
  let shared : int rwprotected = make_rw 0 in
  let events = make_rw [] in

  let log_event s =
    write_rw events (fun lst -> s :: lst)
  in

  (* Long reader *)
  let reader = Thread.create (fun () ->
    read_lock shared.lock;
    log_event "reader-start";
    Thread.delay 0.02;
    let v = shared.data in
    log_event (Printf.sprintf "reader-read=%d" v);
    read_unlock shared.lock;
    log_event "reader-end"
  ) () in

  Thread.delay 0.005;  (* ensure reader starts first *)

  (* Writer must wait for reader to finish *)
  let writer = Thread.create (fun () ->
    log_event "writer-waiting";
    write_lock shared.lock;
    log_event "writer-acquired";
    shared.data <- 42;
    write_unlock shared.lock;
    log_event "writer-done"
  ) () in

  Thread.join reader;
  Thread.join writer;

  let ev = List.rev (read_rw events) in
  Printf.printf "event sequence: %s\n" (String.concat " → " ev);

  Printf.printf "\n=== Concurrent read/write benchmark (approx) ===\n";
  let counter : int rwprotected = make_rw 0 in
  let n_readers = 4 and n_writers = 2 and iters = 500 in
  let all = ref [] in
  for _ = 1 to n_writers do
    all := Thread.create (fun () ->
      for _ = 1 to iters do
        write_rw counter (fun v -> v + 1)
      done
    ) () :: !all
  done;
  for _ = 1 to n_readers do
    all := Thread.create (fun () ->
      let _sum = ref 0 in
      for _ = 1 to iters do
        _sum := !_sum + read_rw counter
      done
    ) () :: !all
  done;
  List.iter Thread.join !all;
  Printf.printf "final counter = %d (expected %d)\n"
    (read_rw counter) (n_writers * iters)
