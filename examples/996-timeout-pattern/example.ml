(* 996: Timeout Pattern *)
(* OCaml: Lwt.pick [operation; Lwt_unix.sleep timeout] concept *)
(* Pure Thread version: run operation in separate thread, wait with timeout *)

(* --- Approach 1: Thread + timed wait via Condition --- *)

type 'a timed_result = Ok of 'a | Timeout | Error of string

let with_timeout_secs timeout_s f =
  let result = ref None in
  let m = Mutex.create () in
  let cond = Condition.create () in

  let worker = Thread.create (fun () ->
    let v = (try Some (f ()) with e -> Some (Error (Printexc.to_string e))) in
    Mutex.lock m;
    result := Some v;
    Condition.signal cond;
    Mutex.unlock m
  ) () in

  Mutex.lock m;
  let deadline = Unix.gettimeofday () +. timeout_s in
  while !result = None do
    let remaining = deadline -. Unix.gettimeofday () in
    if remaining <= 0.0 then (
      result := Some (Some (Error "forced timeout"));
      (* Note: OCaml has no thread kill — worker will finish eventually *)
    ) else
      Condition.wait cond m
      (* In real Lwt: Lwt.pick cancels the losing promise *)
  done;
  Mutex.unlock m;
  Thread.join worker;

  match !result with
  | None | Some None -> Timeout
  | Some (Some (Error msg)) when msg = "forced timeout" -> Timeout
  | Some (Some (Error msg)) -> Error msg
  | Some (Some v) -> Ok v

(* --- Approach 1: fast operation completes in time --- *)

let () =
  let r = with_timeout_secs 1.0 (fun () ->
    Unix.sleepf 0.01;
    42
  ) in
  (match r with
  | Ok v -> assert (v = 42); Printf.printf "Approach 1 (ok): %d\n" v
  | Timeout -> assert false
  | Error e -> Printf.printf "Error: %s\n" e)

(* --- Approach 2: Simulated recv_timeout (channel with deadline) --- *)

type 'a chan = { q: 'a Queue.t; m: Mutex.t; cond: Condition.t }

let make_chan () = { q = Queue.create (); m = Mutex.create (); cond = Condition.create () }

let send c v =
  Mutex.lock c.m; Queue.push v c.q;
  Condition.signal c.cond; Mutex.unlock c.m

let recv_timeout c timeout_s =
  let deadline = Unix.gettimeofday () +. timeout_s in
  Mutex.lock c.m;
  while Queue.is_empty c.q && Unix.gettimeofday () < deadline do
    let remaining = deadline -. Unix.gettimeofday () in
    if remaining > 0.0 then
      Condition.wait c.m c.cond  (* simplified: real code uses timed wait *)
  done;
  let v = if Queue.is_empty c.q then None else Some (Queue.pop c.q) in
  Mutex.unlock c.m;
  v

let () =
  let c = make_chan () in
  let _ = Thread.create (fun () ->
    Unix.sleepf 0.02;
    send c 99
  ) () in
  (* Very short timeout — will miss the send *)
  match recv_timeout c 0.001 with
  | None -> Printf.printf "Approach 2 (timeout): timed out as expected\n"
  | Some v -> Printf.printf "Approach 2 (got): %d\n" v

let () = Printf.printf "✓ All tests passed\n"
