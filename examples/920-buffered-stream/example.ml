(* 920: Buffered Stream — bounded-concurrency parallel map

   OCaml 5.x has domains for true parallelism. For OCaml 4.x we use
   threads (Thread module) with a semaphore via Mutex + Condition.
   The idiomatic OCaml 5 approach uses Domain.spawn + Semaphore.Counting. *)

(* ── Semaphore via Mutex + Condition ──────────────────────────────────────── *)

type semaphore = {
  mutable count : int;
  mutex : Mutex.t;
  cond  : Condition.t;
}

let make_semaphore n = { count = n; mutex = Mutex.create (); cond = Condition.create () }

let acquire sem =
  Mutex.lock sem.mutex;
  while sem.count = 0 do
    Condition.wait sem.cond sem.mutex
  done;
  sem.count <- sem.count - 1;
  Mutex.unlock sem.mutex

let release sem =
  Mutex.lock sem.mutex;
  sem.count <- sem.count + 1;
  Condition.signal sem.cond;
  Mutex.unlock sem.mutex

(* ── buffered_map: apply f to each item with at most `concurrency` in flight ── *)

let buffered_map ~concurrency items f =
  let n = List.length items in
  let results = Array.make n None in
  let sem = make_semaphore concurrency in
  let threads = List.mapi (fun i item ->
    Thread.create (fun () ->
      acquire sem;
      let result = f item in
      release sem;
      results.(i) <- Some result
    ) ()
  ) items in
  List.iter Thread.join threads;
  Array.to_list (Array.map Option.get results)

(* ── Sequential fallback (no threads) ────────────────────────────────────── *)

(* When concurrency = 1 the semaphore makes it effectively sequential *)
let sequential_map items f = List.map f items

let () =
  (* buffered_map preserves order *)
  let result = buffered_map ~concurrency:2 [1; 2; 3; 4; 5] (fun x -> x * 2) in
  assert (result = [2; 4; 6; 8; 10]);

  (* concurrency 1 — effectively sequential *)
  let result2 = buffered_map ~concurrency:1 [1; 2; 3] (fun x -> x + 10) in
  assert (result2 = [11; 12; 13]);

  (* sequential map gives same results *)
  let result3 = sequential_map [1; 2; 3; 4; 5] (fun x -> x * 2) in
  assert (result3 = [2; 4; 6; 8; 10]);

  (* verify ordering even with variable work *)
  let result4 = buffered_map ~concurrency:4 [5; 3; 1; 4; 2]
    (fun x -> Unix.sleepf (float_of_int x *. 0.001); x * x) in
  assert (result4 = [25; 9; 1; 16; 4]);

  print_endline "920-buffered-stream: all tests passed"
