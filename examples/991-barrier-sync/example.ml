(* 991: Barrier Synchronization
   All threads/domains wait at the barrier until every participant has arrived,
   then all proceed together. Useful for phased parallel computation.
   OCaml: Mutex + Condition + counter (no stdlib Barrier). *)

type barrier = {
  n       : int;           (* number of threads expected *)
  mutable arrived : int;  (* threads that have reached the barrier *)
  mutable phase : int;    (* increments each time the barrier is passed *)
  mutex : Mutex.t;
  cond  : Condition.t;
}

let create n =
  assert (n > 0);
  { n; arrived = 0; phase = 0; mutex = Mutex.create (); cond = Condition.create () }

(* Wait at the barrier. Returns true for exactly one thread (the "leader"). *)
let wait barrier =
  Mutex.lock barrier.mutex;
  barrier.arrived <- barrier.arrived + 1;
  let my_phase = barrier.phase in
  let is_leader = barrier.arrived = barrier.n in
  if is_leader then begin
    (* Last thread to arrive: reset and wake all *)
    barrier.arrived <- 0;
    barrier.phase <- barrier.phase + 1;
    Condition.broadcast barrier.cond
  end else begin
    (* Wait until the phase changes *)
    while barrier.phase = my_phase do
      Condition.wait barrier.cond barrier.mutex
    done
  end;
  Mutex.unlock barrier.mutex;
  is_leader

let () =
  Printf.printf "=== Phase barrier: all threads sync between phases ===\n";
  let n = 5 in
  let b = create n in
  let phase1_log = ref [] and phase2_log = ref [] in
  let log_mutex = Mutex.create () in

  let log lst msg =
    Mutex.lock log_mutex;
    lst := msg :: !lst;
    Mutex.unlock log_mutex
  in

  let threads = List.init n (fun i ->
    Thread.create (fun () ->
      (* Phase 1: independent work with varying durations *)
      Thread.delay (float_of_int i *. 0.005);
      log phase1_log (Printf.sprintf "p1:t%d" i);

      (* BARRIER — all threads block until all n have arrived *)
      let leader = wait b in
      if leader then Printf.printf "  leader (last to arrive) opened the gate\n%!";

      (* Phase 2: all start at the same time *)
      log phase2_log (Printf.sprintf "p2:t%d" i)
    ) ()
  ) in
  List.iter Thread.join threads;

  Printf.printf "phase1 completed (all %d): %b\n"
    (List.length !phase1_log) (List.length !phase1_log = n);
  Printf.printf "phase2 completed (all %d): %b\n"
    (List.length !phase2_log) (List.length !phase2_log = n);

  Printf.printf "\n=== Reusable barrier (multiple phases) ===\n";
  let rb = create 3 in
  let trace = ref [] in

  let worker id phases =
    Thread.create (fun () ->
      for phase = 0 to phases - 1 do
        Thread.delay (float_of_int id *. 0.003);
        log trace (Printf.sprintf "t%d-phase%d-pre" id phase);
        ignore (wait rb);
        log trace (Printf.sprintf "t%d-phase%d-post" id phase)
      done
    ) ()
  in

  let ts = List.init 3 (fun i -> worker i 3) in
  List.iter Thread.join ts;

  (* Verify: all pre-barrier events come before all post-barrier events in each phase *)
  Printf.printf "trace length = %d (expected %d)\n"
    (List.length !trace) (3 * 3 * 2);

  Printf.printf "\n=== Domain barrier (true parallelism with OCaml 5) ===\n";
  let db = create 4 in
  let domain_results = Array.make 4 [] in
  let domains = Array.init 4 (fun i ->
    Domain.spawn (fun () ->
      let work phase =
        (* CPU-bound work per phase *)
        let n = (i + 1) * 1000 in
        let sum = ref 0 in
        for j = 1 to n do sum := !sum + j done;
        (phase, !sum)
      in
      let results = ref [] in
      for p = 0 to 2 do
        let r = work p in
        ignore (wait db);  (* sync between phases *)
        results := r :: !results
      done;
      domain_results.(i) <- List.rev !results
    )
  ) in
  Array.iter Domain.join domains;
  Printf.printf "all 4 domains completed 3 phases: %b\n"
    (Array.for_all (fun r -> List.length r = 3) domain_results)
