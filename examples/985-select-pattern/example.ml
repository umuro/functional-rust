(* 985: Select Pattern
   React to whichever of several channels has data first.
   OCaml's Event module provides exactly this: Event.choose [e1; e2; ...]
   synchronises on whichever event is ready first — the CML select primitive.
   We also show a manual poll-based select for plain Queue+Mutex channels. *)

(* --- OCaml Event module: CML-style synchronous select --- *)
let demo_event_select () =
  Printf.printf "=== Event.choose (CML select) ===\n";
  let ch1 = Event.new_channel () in
  let ch2 = Event.new_channel () in
  let ch3 = Event.new_channel () in

  (* Spawn senders with different delays *)
  let _t1 = Thread.create (fun () ->
    Thread.delay 0.02;
    Event.sync (Event.send ch1 "from ch1")
  ) () in
  let _t2 = Thread.create (fun () ->
    Thread.delay 0.005;
    Event.sync (Event.send ch2 "from ch2")
  ) () in
  let _t3 = Thread.create (fun () ->
    Thread.delay 0.01;
    Event.sync (Event.send ch3 "from ch3")
  ) () in

  (* Select: synchronise on the first channel that fires *)
  for round = 1 to 3 do
    let msg = Event.sync (Event.choose [
      Event.receive ch1;
      Event.receive ch2;
      Event.receive ch3;
    ]) in
    Printf.printf "round %d: %s\n" round msg
  done

(* --- Manual select over Queue+Mutex channels with timeout --- *)
type 'a mchan = {
  q     : 'a Queue.t;
  mutex : Mutex.t;
  not_empty : Condition.t;
  mutable closed : bool;
}

let make () =
  { q = Queue.create (); mutex = Mutex.create ();
    not_empty = Condition.create (); closed = false }

let send ch x =
  Mutex.lock ch.mutex;
  Queue.push x ch.q;
  Condition.signal ch.not_empty;
  Mutex.unlock ch.mutex

let try_recv ch =
  Mutex.lock ch.mutex;
  let r = if Queue.is_empty ch.q then None else Some (Queue.pop ch.q) in
  Mutex.unlock ch.mutex;
  r

(* Poll-based select: check each channel in order, retry until one has data *)
let select_poll ?(timeout_ms=1000) channels =
  let deadline = Unix.gettimeofday () +. float_of_int timeout_ms /. 1000.0 in
  let result = ref None in
  while !result = None && Unix.gettimeofday () < deadline do
    (try
      result := List.find_map (fun (i, ch) ->
        match try_recv ch with
        | Some v -> Some (i, v)
        | None   -> None
      ) channels
    with Not_found -> ());
    if !result = None then Thread.delay 0.001  (* yield *)
  done;
  !result

(* --- Timeout channel: sends a value after a delay --- *)
let after_ms ms value =
  let ch = make () in
  let _t = Thread.create (fun () ->
    Thread.delay (float_of_int ms /. 1000.0);
    send ch value
  ) () in
  ch

let () =
  demo_event_select ();

  Printf.printf "\n=== Poll-based select over plain channels ===\n";
  let data_ch   : string mchan = make () in
  let cancel_ch : string mchan = make () in
  let timeout_ch : string mchan = after_ms 50 "timeout" in

  (* Simulate: data arrives after 20ms *)
  let _t = Thread.create (fun () ->
    Thread.delay 0.02;
    send data_ch "data arrived"
  ) () in

  (match select_poll [0, data_ch; 1, cancel_ch; 2, timeout_ch] with
   | Some (0, msg) -> Printf.printf "data channel: %s\n" msg
   | Some (1, msg) -> Printf.printf "cancelled: %s\n" msg
   | Some (2, msg) -> Printf.printf "timed out: %s\n" msg
   | _             -> Printf.printf "nothing\n");

  Printf.printf "\n=== Select with default (non-blocking) ===\n";
  let ch_a : int mchan = make () in
  let ch_b : int mchan = make () in
  send ch_a 42;

  (* Try each channel; fall through to default if none ready *)
  let result = match try_recv ch_a with
    | Some v -> Printf.sprintf "ch_a: %d" v
    | None   -> match try_recv ch_b with
      | Some v -> Printf.sprintf "ch_b: %d" v
      | None   -> "default: nothing ready"
  in
  Printf.printf "%s\n" result;

  Printf.printf "\n=== Merge channels into one ===\n";
  let a : int mchan = make () in
  let b : int mchan = make () in
  let merged : int mchan = make () in

  (* Forwarders *)
  let _fa = Thread.create (fun () ->
    for i = 1 to 3 do Thread.delay 0.005; send a i done
  ) () in
  let _fb = Thread.create (fun () ->
    for i = 10 to 12 do Thread.delay 0.007; send b i done
  ) () in
  let _fwd = Thread.create (fun () ->
    for _ = 1 to 6 do
      (match select_poll [0, a; 1, b] with
       | Some (_, v) -> send merged v
       | None -> ())
    done
  ) () in

  Thread.delay 0.1;
  let acc = ref [] in
  (match try_recv merged with Some v -> acc := v :: !acc | None -> ());
  let rec drain () =
    match try_recv merged with
    | Some v -> acc := v :: !acc; drain ()
    | None -> ()
  in
  drain ();
  Printf.printf "merged (unordered): [%s]\n"
    (String.concat "; " (List.map string_of_int (List.sort compare !acc)))
