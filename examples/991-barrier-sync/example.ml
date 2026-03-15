(* 991: Barrier Synchronization *)
(* Wait until N threads all reach the same point, then continue together *)

type barrier = {
  mutable count: int;
  total: int;
  mutable generation: int;
  m: Mutex.t;
  cond: Condition.t;
}

let make_barrier n = {
  count = 0;
  total = n;
  generation = 0;
  m = Mutex.create ();
  cond = Condition.create ();
}

let barrier_wait b =
  Mutex.lock b.m;
  let gen = b.generation in
  b.count <- b.count + 1;
  if b.count = b.total then begin
    (* Last thread to arrive — wake all waiting *)
    b.count <- 0;
    b.generation <- b.generation + 1;
    Condition.broadcast b.cond;
    Mutex.unlock b.m
  end else begin
    (* Wait until generation changes *)
    while b.generation = gen do
      Condition.wait b.cond b.m
    done;
    Mutex.unlock b.m
  end

(* --- Approach 1: 5 threads synchronize at a barrier --- *)

let () =
  let n = 5 in
  let b = make_barrier n in
  let results = ref [] in
  let m = Mutex.create () in

  let threads = List.init n (fun i ->
    Thread.create (fun () ->
      (* Phase 1: each thread does independent work *)
      Unix.sleepf (float_of_int i *. 0.002);
      Mutex.lock m; results := (Printf.sprintf "p1:%d" i) :: !results; Mutex.unlock m;

      (* BARRIER: wait for all threads *)
      barrier_wait b;

      (* Phase 2: all threads start together *)
      Mutex.lock m; results := (Printf.sprintf "p2:%d" i) :: !results; Mutex.unlock m
    ) ()
  ) in
  List.iter Thread.join threads;

  let p1 = List.filter (fun s -> String.length s > 2 && s.[1] = '1') !results in
  let p2 = List.filter (fun s -> String.length s > 2 && s.[1] = '2') !results in
  assert (List.length p1 = 5);
  assert (List.length p2 = 5);
  Printf.printf "Approach 1 (barrier): %d phase1, %d phase2 items\n"
    (List.length p1) (List.length p2)

(* --- Approach 2: Reusable barrier (multiple rounds) --- *)

let () =
  let n = 3 in
  let b = make_barrier n in
  let round_results = Array.make 2 [] in
  let m = Mutex.create () in

  let threads = List.init n (fun i ->
    Thread.create (fun () ->
      for round = 0 to 1 do
        Mutex.lock m;
        round_results.(round) <- i :: round_results.(round);
        Mutex.unlock m;
        barrier_wait b
      done
    ) ()
  ) in
  List.iter Thread.join threads;
  assert (List.length round_results.(0) = 3);
  assert (List.length round_results.(1) = 3);
  Printf.printf "Approach 2 (reusable barrier): 2 rounds OK\n"

let () = Printf.printf "✓ All tests passed\n"
