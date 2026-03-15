(* 995: Pipeline Stages
   A multi-stage processing pipeline where each stage runs concurrently.
   Data flows through channels: source → parse → validate → transform → sink.
   OCaml: each stage is a thread reading from an input channel and writing
   to an output channel; stages run fully concurrently. *)

(* Bounded channel with backpressure *)
type 'a chan = {
  buf   : 'a option array;
  mutable head : int;
  mutable len  : int;
  cap   : int;
  mutex : Mutex.t;
  not_empty : Condition.t;
  not_full  : Condition.t;
  mutable closed : bool;
}

let make_chan cap =
  { buf = Array.make cap None; head = 0; len = 0; cap;
    mutex = Mutex.create ();
    not_empty = Condition.create (); not_full = Condition.create ();
    closed = false }

let send ch x =
  Mutex.lock ch.mutex;
  while ch.len = ch.cap && not ch.closed do
    Condition.wait ch.not_full ch.mutex
  done;
  if not ch.closed then begin
    ch.buf.((ch.head + ch.len) mod ch.cap) <- Some x;
    ch.len <- ch.len + 1;
    Condition.signal ch.not_empty
  end;
  Mutex.unlock ch.mutex

let recv ch =
  Mutex.lock ch.mutex;
  while ch.len = 0 && not ch.closed do
    Condition.wait ch.not_empty ch.mutex
  done;
  if ch.len = 0 then (Mutex.unlock ch.mutex; None)
  else begin
    let x = ch.buf.(ch.head) in
    ch.buf.(ch.head) <- None;
    ch.head <- (ch.head + 1) mod ch.cap;
    ch.len <- ch.len - 1;
    Condition.signal ch.not_full;
    Mutex.unlock ch.mutex;
    x
  end

let close_chan ch =
  Mutex.lock ch.mutex;
  ch.closed <- true;
  Condition.broadcast ch.not_empty;
  Condition.broadcast ch.not_full;
  Mutex.unlock ch.mutex

(* Run a stage: read from in_ch, apply f, write to out_ch *)
let run_stage name in_ch out_ch f =
  Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv in_ch with
      | None   -> close_chan out_ch; running := false
      | Some x ->
        (match f x with
         | Some y -> send out_ch y
         | None   -> ())  (* filter: None = drop this item *)
    done;
    Printf.printf "stage [%s] done\n%!" name
  ) ()

(* --- Concrete pipeline: string → parsed int → validated → squared → string --- *)
type raw   = string
type parsed = int
type validated = int
type result = string

let parse_stage (s : raw) : parsed option =
  match int_of_string_opt (String.trim s) with
  | Some n -> Some n
  | None   ->
    Printf.printf "  [parse] dropped invalid: %s\n%!" s;
    None

let validate_stage (n : parsed) : validated option =
  if n > 0 && n <= 100 then Some n
  else begin
    Printf.printf "  [validate] dropped out-of-range: %d\n%!" n;
    None
  end

let transform_stage (n : validated) : result option =
  Some (Printf.sprintf "%d^2=%d" n (n * n))

let () =
  Printf.printf "=== 4-stage pipeline ===\n";
  let cap = 8 in
  let ch0 : raw      chan = make_chan cap in  (* source → parse *)
  let ch1 : parsed   chan = make_chan cap in  (* parse → validate *)
  let ch2 : validated chan = make_chan cap in  (* validate → transform *)
  let ch3 : result   chan = make_chan cap in  (* transform → sink *)

  (* Wire up stages *)
  let _s1 = run_stage "parse"    ch0 ch1 parse_stage in
  let _s2 = run_stage "validate" ch1 ch2 validate_stage in
  let _s3 = run_stage "transform" ch2 ch3 transform_stage in

  (* Source: push raw strings *)
  let source = Thread.create (fun () ->
    let items = ["1"; "abc"; "50"; "-5"; "200"; "7"; "  42  "; "0"; "99"] in
    List.iter (send ch0) items;
    close_chan ch0
  ) () in

  (* Sink: collect results *)
  let results = ref [] in
  let sink = Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv ch3 with
      | None   -> running := false
      | Some v -> results := v :: !results
    done
  ) () in

  Thread.join source;
  Thread.join sink;

  Printf.printf "\nResults (in completion order):\n";
  List.iter (Printf.printf "  %s\n") (List.rev !results);

  Printf.printf "\n=== Fan-in pipeline: merge two sources ===\n";
  let src_a : int chan = make_chan 4 in
  let src_b : int chan = make_chan 4 in
  let merged : int chan = make_chan 8 in

  (* Forwarder for src_a *)
  let _fa = Thread.create (fun () ->
    let go = ref true in
    while !go do
      match recv src_a with
      | None   -> go := false
      | Some x -> send merged x
    done
  ) () in
  (* Forwarder for src_b (send negatives) *)
  let _fb = Thread.create (fun () ->
    let go = ref true in
    while !go do
      match recv src_b with
      | None   -> go := false
      | Some x -> send merged (- x)
    done
  ) () in
  (* Close merged when both are done — simple approach: just wait *)
  let _closer = Thread.create (fun () ->
    Thread.delay 0.05;
    close_chan merged
  ) () in

  let _pa = Thread.create (fun () ->
    for i = 1 to 5 do send src_a i done; close_chan src_a
  ) () in
  let _pb = Thread.create (fun () ->
    for i = 1 to 5 do send src_b i done; close_chan src_b
  ) () in

  let collected = ref [] in
  (let go = ref true in
   while !go do
     match recv merged with
     | None   -> go := false
     | Some v -> collected := v :: !collected
   done);
  Printf.printf "merged (sorted): [%s]\n"
    (String.concat "; " (List.map string_of_int (List.sort compare !collected)))
