(* 343: Cancellation Token *)
(* OCaml: use Atomic or ref for cooperative cancellation *)

(* Approach 1: Simple ref-based cancellation *)
let make_token () = ref false
let cancel token = token := true
let is_cancelled token = !token

let worker token name =
  let count = ref 0 in
  while not (is_cancelled token) && !count < 100 do
    incr count
  done;
  Printf.sprintf "%s did %d iterations" name !count

(* Approach 2: Atomic for thread safety *)
let make_atomic_token () = Atomic.make false
let atomic_cancel token = Atomic.set token true
let atomic_is_cancelled token = Atomic.get token

(* Approach 3: Multi-worker cancellation *)
let run_workers n =
  let token = make_token () in
  let results = Array.make n "" in
  let threads = Array.init n (fun i ->
    Thread.create (fun () ->
      results.(i) <- worker token (Printf.sprintf "worker-%d" i)
    ) ()
  ) in
  Unix.sleepf 0.001;
  cancel token;
  Array.iter Thread.join threads;
  Array.to_list results

(* Tests *)
let () =
  let token = make_token () in
  assert (not (is_cancelled token));
  cancel token;
  assert (is_cancelled token);
  let result = worker (make_token ()) "test" in
  assert (String.length result > 0);
  Printf.printf "✓ All tests passed\n"
