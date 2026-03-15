(* 981: Sequential Async Chain *)
(* OCaml: let* x = ... in let* y = ... using ppx_let or Lwt.( let* ) *)

type 'a future = unit -> 'a

let return_ x : 'a future = fun () -> x
let bind fut k = fun () -> k (fut ()) ()
let run f = f ()

(* Simulated let* (monadic bind) *)
let ( let* ) = bind

(* --- Approach 1: Sequential chain with let* --- *)

let fetch_user_id () = return_ 42
let fetch_user_name _id = return_ "Alice"
let fetch_user_email _name = return_ "alice@example.com"

let full_lookup () =
  let* id = fetch_user_id () in
  let* name = fetch_user_name id in
  let* email = fetch_user_email name in
  return_ (id, name, email)

let () =
  let (id, name, email) = run (full_lookup ()) in
  assert (id = 42);
  assert (name = "Alice");
  assert (email = "alice@example.com");
  Printf.printf "Approach 1 (let* chain): id=%d name=%s email=%s\n" id name email

(* --- Approach 2: Accumulating values through chain --- *)

let step1 x = return_ (x + 10)
let step2 x = return_ (x * 2)
let step3 x = return_ (x - 5)

let pipeline_seq input =
  let* a = step1 input in
  let* b = step2 a in
  let* c = step3 b in
  return_ (input, a, b, c)

let () =
  let (orig, a, b, c) = run (pipeline_seq 5) in
  (* 5 -> 15 -> 30 -> 25 *)
  assert (orig = 5);
  assert (a = 15);
  assert (b = 30);
  assert (c = 25);
  Printf.printf "Approach 2 (pipeline): %d->%d->%d->%d\n" orig a b c

(* --- Approach 3: Short-circuit with error-aware sequence --- *)

type ('a, 'e) result_future = unit -> ('a, 'e) result

let ok x : ('a, 'e) result_future = fun () -> Ok x
let fail e : ('a, 'e) result_future = fun () -> Error e
let run_r f = f ()

let ( let*? ) (fut : ('a, 'e) result_future) k =
  fun () -> match fut () with
    | Ok v -> k v ()
    | Error e -> Error e

let guarded_div a b =
  if b = 0 then fail "division by zero"
  else ok (a / b)

let safe_pipeline () =
  let*? x = ok 100 in
  let*? y = guarded_div x 4 in
  let*? z = guarded_div y 5 in
  ok z

let bad_pipeline () =
  let*? x = ok 100 in
  let*? _ = guarded_div x 0 in  (* short-circuits here *)
  ok 999

let () =
  (match run_r (safe_pipeline ()) with
  | Ok v -> assert (v = 5); Printf.printf "Approach 3 (safe): %d\n" v
  | Error _ -> assert false);
  (match run_r (bad_pipeline ()) with
  | Ok _ -> assert false
  | Error e -> Printf.printf "Approach 3 (error): %s\n" e)

let () = Printf.printf "✓ All tests passed\n"
