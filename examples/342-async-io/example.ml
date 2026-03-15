(* 342: Async I/O Concepts — polling vs blocking, parallel reads.
   OCaml 5 equivalent: Domain for parallelism, Thread + Mutex + channels for
   concurrent I/O, and a hand-rolled Poll type for cooperative scheduling. *)

(* ── Approach 1: Simulated blocking read (sequential) ──────────────────────── *)

let blocking_read () =
  (* Simulate I/O latency with Thread.delay *)
  Thread.delay 0.01;
  "data from blocking read"

(* ── Approach 2: Parallel reads via Domain + mutex-protected result list ────── *)

let parallel_reads () =
  let results = ref [] in
  let mutex   = Mutex.create () in
  let push s  =
    Mutex.lock mutex;
    results := s :: !results;
    Mutex.unlock mutex
  in
  let d1 = Domain.spawn (fun () ->
    Thread.delay 0.01;
    push "result1"
  ) in
  let d2 = Domain.spawn (fun () ->
    Thread.delay 0.01;
    push "result2"
  ) in
  Domain.join d1;
  Domain.join d2;
  !results

(* ── Approach 3: Poll type — cooperative scheduling / future simulation ──────── *)

(* A Poll represents a computation that may or may not be ready yet.
   This mirrors Rust's std::task::Poll / Future model. *)
type 'a poll = Ready of 'a | Pending

(* Simulate a future that becomes ready after n calls *)
let make_future total_steps =
  let counter = ref 0 in
  fun () ->
    if !counter >= total_steps then Ready "done"
    else begin incr counter; Pending end

(* A simple "executor" that drives a poll-based future to completion *)
let run_future poll_fn =
  let steps = ref 0 in
  let result = ref None in
  while !result = None do
    incr steps;
    match poll_fn () with
    | Ready v -> result := Some v
    | Pending -> ()
  done;
  (!steps, Option.get !result)

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* Blocking read *)
  let data = blocking_read () in
  Printf.printf "blocking_read = %S\n" data;

  (* Parallel reads *)
  let results = parallel_reads () in
  Printf.printf "parallel_reads count = %d\n" (List.length results);
  assert (List.mem "result1" results);
  assert (List.mem "result2" results);
  Printf.printf "parallel_reads contains result1=%b result2=%b\n"
    (List.mem "result1" results) (List.mem "result2" results);

  (* Poll simulation *)
  let future = make_future 3 in
  assert (future () = Pending);
  assert (future () = Pending);
  assert (future () = Pending);
  assert (future () = Ready "done");
  Printf.printf "poll: Pending → Pending → Pending → Ready\n";

  (* Run a fresh future with the executor *)
  let (steps, value) = run_future (make_future 3) in
  Printf.printf "executor ran future in %d steps, result = %S\n" steps value
