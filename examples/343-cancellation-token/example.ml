(* 343: Cancellation Token
   Cooperative cancellation using Atomic_bool across domains/threads. *)

(* A cancellation token is just a shared atomic boolean *)
let make_token () = Atomic.make false

let cancel token = Atomic.set token true

let is_cancelled token = Atomic.get token

(* Worker that checks the token and counts iterations *)
let worker token name =
  let count = ref 0 in
  while not (is_cancelled token) && !count < 1_000_000 do
    incr count
  done;
  Printf.sprintf "%s did %d iterations" name !count

(* Spawn multiple workers sharing one token, then cancel them *)
let run_workers n =
  let token = make_token () in
  let domains =
    List.init n (fun i ->
      let name = Printf.sprintf "worker-%d" i in
      Domain.spawn (fun () -> worker token name))
  in
  (* Let them run briefly, then cancel *)
  Unix.sleepf 0.001;
  cancel token;
  List.map Domain.join domains

let () =
  (* Basic token test *)
  let t = make_token () in
  assert (not (is_cancelled t));
  cancel t;
  assert (is_cancelled t);
  Printf.printf "Token cancel: ok\n%!";

  (* Single worker *)
  let t2 = make_token () in
  let d = Domain.spawn (fun () -> worker t2 "alpha") in
  Unix.sleepf 0.001;
  cancel t2;
  let result = Domain.join d in
  Printf.printf "Worker result: %s\n%!" result;

  (* Multi-worker *)
  let results = run_workers 3 in
  List.iter (fun r -> Printf.printf "  %s\n%!" r) results;
  Printf.printf "All %d workers cancelled.\n%!" (List.length results)
