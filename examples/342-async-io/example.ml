(* 342: Async I/O Concepts *)
(* OCaml: simulate with threads and channels *)

(* Approach 1: Blocking I/O simulation *)
let blocking_read () =
  Unix.sleepf 0.01;
  "data from blocking read"

(* Approach 2: Threaded I/O *)
let parallel_reads () =
  let ch = Event.new_channel () in
  let t1 = Thread.create (fun () ->
    Unix.sleepf 0.01;
    Event.sync (Event.send ch "result1")
  ) () in
  let t2 = Thread.create (fun () ->
    Unix.sleepf 0.01;
    Event.sync (Event.send ch "result2")
  ) () in
  let r1 = Event.sync (Event.receive ch) in
  let r2 = Event.sync (Event.receive ch) in
  Thread.join t1;
  Thread.join t2;
  [r1; r2]

(* Approach 3: Simple polling simulation *)
type 'a poll_result = Ready of 'a | Pending

let simulate_poll counter =
  if !counter >= 3 then (counter := 0; Ready "done")
  else (incr counter; Pending)

(* Tests *)
let () =
  assert (blocking_read () = "data from blocking read");
  let counter = ref 0 in
  assert (simulate_poll counter = Pending);
  assert (simulate_poll counter = Pending);
  assert (simulate_poll counter = Ready "done");
  Printf.printf "✓ All tests passed\n"
