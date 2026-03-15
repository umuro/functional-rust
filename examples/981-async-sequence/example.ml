(* 981: Async Sequence (Sequential Async Chain)
   Chain async computations so each step begins only after the previous completes.
   OCaml: monadic bind on futures — equivalent to Rust's sequential .await.
   Demonstrates let-binding style and error propagation. *)

(* Result-future type: captures both success and error *)
type ('a, 'e) result_future = {
  mutable result : ('a, 'e) result option;
  mutex : Mutex.t;
  cond  : Condition.t;
}

let make () =
  { result = None; mutex = Mutex.create (); cond = Condition.create () }

let complete fut r =
  Mutex.lock fut.mutex;
  fut.result <- Some r;
  Condition.broadcast fut.cond;
  Mutex.unlock fut.mutex

let await fut =
  Mutex.lock fut.mutex;
  while fut.result = None do Condition.wait fut.cond fut.mutex done;
  let r = Option.get fut.result in
  Mutex.unlock fut.mutex;
  r

(* Spawn an async task returning Result *)
let async_ok f =
  let fut = make () in
  let _t = Thread.create (fun () ->
    let r = try Ok (f ()) with e -> Error e in
    complete fut r
  ) () in
  fut

(* Monadic bind: run f only if previous step succeeded *)
let ( let* ) fut f =
  match await fut with
  | Error e  -> let r = make () in complete r (Error e); r
  | Ok value -> f value

(* Lift a pure value into a future *)
let return x =
  let fut = make () in
  complete fut (Ok x);
  fut

(* --- Simulated async operations --- *)
let fetch_user_id () =
  Thread.delay 0.005;
  42

let fetch_user_name id =
  Thread.delay 0.005;
  if id = 42 then "Alice" else failwith "unknown user"

let fetch_email name =
  Thread.delay 0.005;
  Printf.sprintf "%s@example.com" (String.lowercase_ascii name)

let safe_div a b =
  if b = 0 then Error "division by zero" else Ok (a / b)

let () =
  Printf.printf "=== Sequential async chain ===\n";

  (* Step 1: fetch id → Step 2: fetch name → Step 3: fetch email *)
  let pipeline =
    let* id   = async_ok fetch_user_id in
    let* name = async_ok (fun () -> fetch_user_name id) in
    let* email = async_ok (fun () -> fetch_email name) in
    return (id, name, email)
  in
  (match await pipeline with
   | Ok (id, name, email) ->
     Printf.printf "id=%d  name=%s  email=%s\n" id name email
   | Error e -> Printf.printf "error: %s\n" (Printexc.to_string e));

  (* Arithmetic pipeline *)
  Printf.printf "\n=== Arithmetic pipeline ===\n";
  let arith =
    let* a = async_ok (fun () -> 10) in
    let* b = async_ok (fun () -> a + 5) in
    let* c = async_ok (fun () -> b * 2) in
    return (a, b, c)
  in
  (match await arith with
   | Ok (a, b, c) -> Printf.printf "10 → +5=%d → *2=%d\n" b c; ignore a
   | Error e -> Printf.printf "error: %s\n" (Printexc.to_string e));

  (* Error short-circuits the chain *)
  Printf.printf "\n=== Error short-circuit ===\n";
  let failing =
    let* x  = async_ok (fun () -> 100) in
    let* _y = async_ok (fun () -> failwith "oops") in
    return (x + 1)  (* never reached *)
  in
  (match await failing with
   | Ok v   -> Printf.printf "ok: %d\n" v
   | Error e -> Printf.printf "short-circuited at error: %s\n" (Printexc.to_string e));

  (* Collect results: sequential fold over a list *)
  Printf.printf "\n=== Sequential fold ===\n";
  let steps = [10; 20; 30; 40] in
  let sum_future = List.fold_left (fun acc_fut n ->
    let* acc = acc_fut in
    async_ok (fun () ->
      Thread.delay 0.002;
      acc + n)
  ) (return 0) steps in
  (match await sum_future with
   | Ok total -> Printf.printf "sequential sum = %d\n" total
   | Error e  -> Printf.printf "error: %s\n" (Printexc.to_string e))
