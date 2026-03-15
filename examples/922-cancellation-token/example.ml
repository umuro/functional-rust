(* 922: Cancellation Token — cooperative cancellation via shared atomic flag

   OCaml uses a mutable ref behind a Mutex for thread-safe cancellation.
   In single-threaded code a plain ref suffices; in multi-threaded code
   we protect it with Atomic.t (OCaml 5.x) or a Mutex. *)

(* ── Cancellation Token ───────────────────────────────────────────────────── *)

type cancellation_token = {
  mutable cancelled : bool;
  mutex : Mutex.t;
}

let make_token () = { cancelled = false; mutex = Mutex.create () }

let cancel token =
  Mutex.lock token.mutex;
  token.cancelled <- true;
  Mutex.unlock token.mutex

let is_cancelled token =
  Mutex.lock token.mutex;
  let v = token.cancelled in
  Mutex.unlock token.mutex;
  v

(* ── Long-running task with cancellation check ──────────────────────────── *)

let long_task token steps =
  let rec go i =
    if i >= steps then Ok (Printf.sprintf "completed all %d steps" steps)
    else if is_cancelled token then
      Error (Printf.sprintf "cancelled at step %d" i)
    else begin
      (* simulate work without actually sleeping *)
      ignore (i * i);
      go (i + 1)
    end
  in
  go 0

(* Sum with periodic cancellation check *)
let cancellable_sum token data =
  let arr = Array.of_list data in
  let n = Array.length arr in
  let rec go i acc =
    if i >= n then Some acc
    else if i mod 1000 = 0 && is_cancelled token then None
    else go (i + 1) (acc + arr.(i))
  in
  go 0 0

let () =
  (* token starts not cancelled *)
  let t = make_token () in
  assert (not (is_cancelled t));

  (* cancel sets flag *)
  cancel t;
  assert (is_cancelled t);

  (* task completes without cancel *)
  let t2 = make_token () in
  let result = long_task t2 5 in
  assert (result = Ok "completed all 5 steps");

  (* task cancelled immediately *)
  let t3 = make_token () in
  cancel t3;
  let result2 = long_task t3 100 in
  assert (match result2 with Error _ -> true | Ok _ -> false);

  (* cancellable_sum — completes without cancel *)
  let t4 = make_token () in
  let data = List.init 5000 (fun i -> i + 1) in
  let s = cancellable_sum t4 data in
  assert (s = Some (5000 * 5001 / 2));

  (* cancellable_sum — aborted *)
  let t5 = make_token () in
  cancel t5;
  let s2 = cancellable_sum t5 (List.init 2000 (fun i -> i)) in
  assert (s2 = None);

  (* token cloning: independent copies share the flag via the same record ref *)
  let t6 = make_token () in
  let t6_copy = t6 in   (* OCaml records are mutable refs, not Copy *)
  cancel t6_copy;
  assert (is_cancelled t6);  (* shared — same record *)

  print_endline "922-cancellation-token: all tests passed"
