(* 995: N-Stage Streaming Pipeline *)
(* Each stage is a thread + channel. Data flows item by item *)

type 'a chan = { q: 'a Queue.t; m: Mutex.t; cond: Condition.t; mutable closed: bool }

let make_chan () = { q = Queue.create (); m = Mutex.create ();
                     cond = Condition.create (); closed = false }

let send c v =
  Mutex.lock c.m; Queue.push v c.q;
  Condition.signal c.cond; Mutex.unlock c.m

let close_chan c =
  Mutex.lock c.m; c.closed <- true;
  Condition.broadcast c.cond; Mutex.unlock c.m

let recv c =
  Mutex.lock c.m;
  while Queue.is_empty c.q && not c.closed do
    Condition.wait c.cond c.m done;
  let v = if Queue.is_empty c.q then None else Some (Queue.pop c.q) in
  Mutex.unlock c.m; v

(* Stage: thread that maps f over incoming items *)
let make_stage in_c f =
  let out_c = make_chan () in
  let _ = Thread.create (fun () ->
    let rec loop () = match recv in_c with
      | None -> close_chan out_c
      | Some v -> send out_c (f v); loop ()
    in loop ()
  ) () in
  out_c

(* Filter stage *)
let make_filter in_c pred =
  let out_c = make_chan () in
  let _ = Thread.create (fun () ->
    let rec loop () = match recv in_c with
      | None -> close_chan out_c
      | Some v -> if pred v then send out_c v; loop ()
    in loop ()
  ) () in
  out_c

(* --- Build a pipeline: source -> stages -> sink --- *)

let () =
  let source = make_chan () in

  (* Pipeline: parse int -> square -> filter even -> to string *)
  let c1 = make_stage source (fun x -> x * x) in
  let c2 = make_filter c1 (fun x -> x mod 2 = 0) in
  let c3 = make_stage c2 string_of_int in

  (* Feed data *)
  let producer = Thread.create (fun () ->
    List.iter (send source) [1;2;3;4;5;6;7;8;9;10];
    close_chan source
  ) () in

  (* Collect *)
  let results = ref [] in
  let rec collect () = match recv c3 with
    | None -> ()
    | Some v -> results := v :: !results; collect ()
  in
  collect ();
  Thread.join producer;

  let results = List.rev !results in
  (* Even squares of 1..10: 4,16,36,64,100 *)
  assert (results = ["4";"16";"36";"64";"100"]);
  Printf.printf "Approach 1 (filter pipeline): [%s]\n"
    (String.concat "; " results)

let () = Printf.printf "✓ All tests passed\n"
