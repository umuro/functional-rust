(* 984: Channel Pipeline
   Connect stages via channels: producer → stage1 → stage2 → consumer.
   Each stage runs in its own thread and passes results to the next channel.
   OCaml: unbounded channels (Queue + Mutex + Condition). *)

type 'a chan = {
  q : 'a Queue.t;
  mutex : Mutex.t;
  not_empty : Condition.t;
  mutable closed : bool;
}

let make_chan () =
  { q = Queue.create (); mutex = Mutex.create ();
    not_empty = Condition.create (); closed = false }

let send ch x =
  Mutex.lock ch.mutex;
  Queue.push x ch.q;
  Condition.signal ch.not_empty;
  Mutex.unlock ch.mutex

let recv ch =
  Mutex.lock ch.mutex;
  while Queue.is_empty ch.q && not ch.closed do
    Condition.wait ch.not_empty ch.mutex
  done;
  let r = if Queue.is_empty ch.q then None else Some (Queue.pop ch.q) in
  Mutex.unlock ch.mutex;
  r

let close ch =
  Mutex.lock ch.mutex;
  ch.closed <- true;
  Condition.broadcast ch.not_empty;
  Mutex.unlock ch.mutex

(* Run a transformation stage: read from in_ch, transform, write to out_ch *)
let stage f in_ch out_ch =
  Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv in_ch with
      | None   -> close out_ch; running := false
      | Some x -> send out_ch (f x)
    done
  ) ()

(* Run a sink stage: read from in_ch and call f for each item *)
let sink f in_ch =
  Thread.create (fun () ->
    let running = ref true in
    while !running do
      match recv in_ch with
      | None   -> running := false
      | Some x -> f x
    done
  ) ()

(* Build a pipeline from a list of functions *)
let pipeline source_items fns =
  let n = List.length fns in
  (* Create n+1 channels: one before each stage and one after the last *)
  let channels = Array.init (n + 1) (fun _ -> make_chan ()) in

  (* Producer: push items into the first channel *)
  let _prod = Thread.create (fun () ->
    List.iter (send channels.(0)) source_items;
    close channels.(0)
  ) () in

  (* Wire up transformation stages *)
  let _stages = List.mapi (fun i f ->
    stage f channels.(i) channels.(i + 1)
  ) fns in

  (* Return the output channel *)
  channels.(n)

let () =
  Printf.printf "=== Basic pipeline: numbers → square → string → uppercase ===\n";
  let numbers = List.init 8 (fun i -> i + 1) in

  let out_ch = pipeline numbers [
    (fun x -> x * x);                        (* stage 1: square *)
  ] in

  (* Collect results from output channel *)
  let results = ref [] in
  let running = ref true in
  while !running do
    match recv out_ch with
    | None   -> running := false
    | Some v -> results := v :: !results
  done;
  let sorted = List.sort compare (List.rev !results) in
  Printf.printf "squares: [%s]\n"
    (String.concat "; " (List.map string_of_int sorted));

  Printf.printf "\n=== Multi-stage string pipeline ===\n";
  let ch0 : string chan = make_chan () in

  (* Stage 1: trim whitespace *)
  let ch1 : string chan = make_chan () in
  let _s1 = stage String.trim ch0 ch1 in

  (* Stage 2: uppercase *)
  let ch2 : string chan = make_chan () in
  let _s2 = stage String.uppercase_ascii ch1 ch2 in

  (* Stage 3: add prefix *)
  let ch3 : string chan = make_chan () in
  let _s3 = stage (fun s -> ">>>" ^ s) ch2 ch3 in

  (* Producer *)
  let items = ["  hello  "; " world "; "  ocaml  "; " pipeline "] in
  let _prod = Thread.create (fun () ->
    List.iter (send ch0) items;
    close ch0
  ) () in

  (* Collect *)
  let out = ref [] in
  let r = ref true in
  while !r do
    match recv ch3 with
    | None   -> r := false
    | Some s -> out := s :: !out
  done;
  List.iter (Printf.printf "  %s\n") (List.rev !out);

  Printf.printf "\n=== Fan-out: broadcast to multiple consumers ===\n";
  let src : int chan = make_chan () in
  let c1 : int chan = make_chan () in
  let c2 : int chan = make_chan () in

  (* Broadcast: copy each item to both consumers *)
  let _broadcast = Thread.create (fun () ->
    let go = ref true in
    while !go do
      match recv src with
      | None   -> close c1; close c2; go := false
      | Some x -> send c1 x; send c2 (x * 10)
    done
  ) () in

  let results1 = ref [] and results2 = ref [] in
  let t1 = sink (fun x -> results1 := x :: !results1) c1 in
  let t2 = sink (fun x -> results2 := x :: !results2) c2 in

  List.iter (send src) [1;2;3;4;5];
  close src;
  Thread.join t1; Thread.join t2;

  Printf.printf "consumer1: [%s]\n"
    (String.concat "; " (List.map string_of_int (List.sort compare !results1)));
  Printf.printf "consumer2 (×10): [%s]\n"
    (String.concat "; " (List.map string_of_int (List.sort compare !results2)))
