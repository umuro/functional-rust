(* 979: Future Basics
   OCaml 5 provides Domain for true parallelism and Thread for concurrency.
   The nearest equivalent to Rust futures is a "promise/future" pair built
   on Mutex + Condition variable, which mirrors async computation completion.
   We also show Domain-based parallel futures for CPU work. *)

(* --- Simple promise/future over Thread + Mutex --- *)
type 'a state =
  | Pending
  | Resolved of 'a
  | Failed of exn

type 'a future = {
  mutable state : 'a state;
  mutex : Mutex.t;
  cond  : Condition.t;
}

let create_future () =
  { state = Pending; mutex = Mutex.create (); cond = Condition.create () }

let resolve fut value =
  Mutex.lock fut.mutex;
  fut.state <- Resolved value;
  Condition.broadcast fut.cond;
  Mutex.unlock fut.mutex

let reject fut exn =
  Mutex.lock fut.mutex;
  fut.state <- Failed exn;
  Condition.broadcast fut.cond;
  Mutex.unlock fut.mutex

(* Block until the future resolves *)
let await fut =
  Mutex.lock fut.mutex;
  while fut.state = Pending do
    Condition.wait fut.cond fut.mutex
  done;
  let result = fut.state in
  Mutex.unlock fut.mutex;
  match result with
  | Resolved v -> v
  | Failed e   -> raise e
  | Pending    -> assert false

(* Spawn an async computation — runs in a background thread *)
let async f =
  let fut = create_future () in
  let _thread = Thread.create (fun () ->
    match f () with
    | v   -> resolve fut v
    | exception e -> reject fut e
  ) () in
  fut

(* Combinators *)
let map fut f =
  async (fun () -> f (await fut))

let bind fut f =
  async (fun () -> await (f (await fut)))

(* Wait for both futures *)
let both fa fb =
  async (fun () -> (await fa, await fb))

(* Race: return whichever resolves first *)
let first_of futures =
  let winner = create_future () in
  List.iter (fun fut ->
    let _t = Thread.create (fun () ->
      (try resolve winner (await fut)
       with _ -> ())
    ) () in ()
  ) futures;
  winner

let () =
  Printf.printf "=== Future basics ===\n";

  (* Simple async computation *)
  let f1 = async (fun () ->
    Thread.delay 0.01;
    42
  ) in
  let f2 = async (fun () ->
    Thread.delay 0.005;
    "hello"
  ) in
  Printf.printf "f1 = %d\n" (await f1);
  Printf.printf "f2 = %s\n" (await f2);

  (* map: transform the result *)
  let f3 = map f1 (fun x -> x * 2) in
  Printf.printf "f1 mapped *2 = %d\n" (await f3);

  (* bind: chain async computations *)
  let f4 = bind (async (fun () -> 10)) (fun n ->
    async (fun () -> n + 5)
  ) in
  Printf.printf "bind 10 + 5 = %d\n" (await f4);

  (* both: parallel execution *)
  let start = Unix.gettimeofday () in
  let fa = async (fun () -> Thread.delay 0.02; "A done") in
  let fb = async (fun () -> Thread.delay 0.02; "B done") in
  let (ra, rb) = await (both fa fb) in
  let elapsed = Unix.gettimeofday () -. start in
  Printf.printf "both: (%s, %s) in %.0fms (ran in parallel)\n"
    ra rb (elapsed *. 1000.0);

  (* Error handling *)
  let failing = async (fun () -> failwith "something went wrong") in
  (try ignore (await failing)
   with Failure msg -> Printf.printf "caught: %s\n" msg);

  Printf.printf "\n=== Domain-based parallel futures (OCaml 5) ===\n";
  (* Domains for true CPU parallelism *)
  let compute n =
    Domain.spawn (fun () ->
      (* simulate work *)
      let sum = ref 0 in
      for i = 1 to n do sum := !sum + i done;
      !sum
    )
  in
  let d1 = compute 1000 in
  let d2 = compute 2000 in
  Printf.printf "sum 1..1000 = %d\n" (Domain.join d1);
  Printf.printf "sum 1..2000 = %d\n" (Domain.join d2)
