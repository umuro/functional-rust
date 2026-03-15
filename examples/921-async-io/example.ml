(* 921: Async I/O — offloading blocking I/O to threads, non-blocking sockets

   OCaml's idiomatic async I/O: threads (Thread), non-blocking sockets via
   Unix module, or the Lwt / Async libraries. Here we show the stdlib approach:
   spawn an I/O task in a Thread, communicate via Mutex-protected ref. *)

(* ── Thread-based async task ─────────────────────────────────────────────── *)

(* Spawn a thunk in a thread, return a "future" (thunk → result via ref) *)
let spawn_io_task f =
  let result = ref None in
  let mutex = Mutex.create () in
  let cond = Condition.create () in
  let _ = Thread.create (fun () ->
    let v = f () in
    Mutex.lock mutex;
    result := Some v;
    Condition.signal cond;
    Mutex.unlock mutex
  ) () in
  (* Return a function to block and collect the result *)
  fun () ->
    Mutex.lock mutex;
    while !result = None do
      Condition.wait cond mutex
    done;
    let v = Option.get !result in
    Mutex.unlock mutex;
    v

(* ── Text processing ─────────────────────────────────────────────────────── *)

let process_text text =
  let lines = String.split_on_char '\n' text |> List.length in
  let words  = String.split_on_char ' ' text
               |> List.concat_map (String.split_on_char '\n')
               |> List.filter (fun s -> s <> "")
               |> List.length in
  let chars  = String.length text in
  (lines, words, chars)

(* ── Write to buffer ─────────────────────────────────────────────────────── *)

(* OCaml's Buffer is the idiomatic mutable string builder *)
let write_to_buf buf data =
  Buffer.add_string buf data;
  String.length data

(* ── Non-blocking socket listener ────────────────────────────────────────── *)

let make_nonblocking_listener () =
  let sock = Unix.socket Unix.PF_INET Unix.SOCK_STREAM 0 in
  Unix.setsockopt sock Unix.SO_REUSEADDR true;
  Unix.bind sock (Unix.ADDR_INET (Unix.inet_addr_loopback, 0));
  Unix.listen sock 1;
  Unix.set_nonblock sock;
  sock

let () =
  (* process_text counts *)
  let (lines, words, chars) = process_text "hello world\nfoo" in
  assert (lines = 2);
  assert (words = 3);
  assert (chars = 15);

  (* spawn_io_task delivers value *)
  let await = spawn_io_task (fun () -> 42) in
  assert (await () = 42);

  (* write_to_buf *)
  let buf = Buffer.create 16 in
  let n = write_to_buf buf "hello" in
  assert (n = 5);
  assert (Buffer.contents buf = "hello");

  (* non-blocking socket: accept immediately returns an error *)
  let sock = make_nonblocking_listener () in
  (try
    ignore (Unix.accept sock);
    failwith "expected EAGAIN"
  with Unix.Unix_error (Unix.EAGAIN, _, _) -> ()
     | Unix.Unix_error (Unix.EWOULDBLOCK, _, _) -> ());
  Unix.close sock;

  (* Async string read *)
  let await2 = spawn_io_task (fun () ->
    (* simulate I/O latency *)
    Thread.delay 0.001;
    "async result"
  ) in
  assert (await2 () = "async result");

  print_endline "921-async-io: all tests passed"
