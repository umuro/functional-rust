(* 754: Testing Async Code — OCaml with Lwt-style simulation using threads *)
(* We use Thread + Mutex to model async behavior in stdlib OCaml *)

let process_message msg =
  (* Simulate async work *)
  Thread.delay 0.001;  (* 1ms *)
  String.uppercase_ascii msg

(* Worker: reads from inbox, writes to outbox *)
let start_worker inbox outbox =
  let running = ref true in
  let thread = Thread.create (fun () ->
    while !running do
      match Queue.take_opt inbox with
      | Some msg ->
        let result = process_message msg in
        Queue.push result outbox
      | None ->
        Thread.delay 0.0001
    done
  ) () in
  (thread, running)

let () =
  let inbox  = Queue.create () in
  let outbox = Queue.create () in
  let (thread, running) = start_worker inbox outbox in

  (* Send messages *)
  Queue.push "hello" inbox;
  Queue.push "world" inbox;

  (* Wait for results *)
  let deadline = Unix.gettimeofday () +. 1.0 in
  let results = ref [] in
  while List.length !results < 2 && Unix.gettimeofday () < deadline do
    (match Queue.take_opt outbox with
    | Some r -> results := r :: !results
    | None   -> Thread.delay 0.001)
  done;

  running := false;
  Thread.join thread;

  let sorted = List.sort compare !results in
  List.iter (Printf.printf "Result: %s\n") sorted;
  assert (List.mem "HELLO" sorted);
  assert (List.mem "WORLD" sorted);
  Printf.printf "Async-style test passed!\n"
