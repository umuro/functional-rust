(* 458: Barrier Synchronization
   A barrier makes N domains wait until all N have reached the meeting
   point — analogous to Rust's std::sync::Barrier.
   OCaml has no built-in Barrier, so we implement one with Mutex + Condition. *)

type barrier = {
  n       : int;
  mutex   : Mutex.t;
  cond    : Condition.t;
  waiting : int ref;
  (* generation counter prevents re-use issues *)
  gen     : int ref;
}

let make_barrier n = {
  n; mutex = Mutex.create (); cond = Condition.create ();
  waiting = ref 0; gen = ref 0;
}

(* Returns true for exactly one domain per generation (the "leader") *)
let wait barrier =
  Mutex.lock barrier.mutex;
  barrier.waiting := !(barrier.waiting) + 1;
  let my_gen = !(barrier.gen) in
  let is_leader =
    if !(barrier.waiting) = barrier.n then begin
      (* Last to arrive — advance generation and wake everyone *)
      barrier.waiting := 0;
      barrier.gen := my_gen + 1;
      Condition.broadcast barrier.cond;
      true
    end else begin
      (* Wait until the generation advances *)
      while !(barrier.gen) = my_gen do
        Condition.wait barrier.cond barrier.mutex
      done;
      false
    end
  in
  Mutex.unlock barrier.mutex;
  is_leader

let () =
  (* All N domains must arrive before any continues *)
  let n = 4 in
  let b = make_barrier n in
  let counter = Atomic.make 0 in

  let domains = List.init n (fun _ ->
    Domain.spawn (fun () ->
      ignore (Atomic.fetch_and_add counter 1);
      let _leader = wait b in
      (* After the barrier, all N increments are visible *)
      assert (Atomic.get counter = n)))
  in
  List.iter Domain.join domains;
  Printf.printf "barrier: all %d domains passed, counter=%d\n%!" n (Atomic.get counter);

  (* Exactly one leader per generation *)
  let n2 = 5 in
  let b2 = make_barrier n2 in
  let leaders = Atomic.make 0 in

  let domains2 = List.init n2 (fun _ ->
    Domain.spawn (fun () ->
      if wait b2 then
        ignore (Atomic.fetch_and_add leaders 1)))
  in
  List.iter Domain.join domains2;
  assert (Atomic.get leaders = 1);
  Printf.printf "barrier leader count: %d\n%!" (Atomic.get leaders)
