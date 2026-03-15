(* 982: Async Join
   Run multiple async tasks concurrently and wait for all (join_all)
   or the first one to complete (select/race).
   OCaml: Domain.spawn for parallelism; Thread + Mutex + Condition for concurrency. *)

(* Reusable future primitives *)
type 'a fut = {
  mutable v : 'a option;
  mutable e : exn option;
  m : Mutex.t;
  c : Condition.t;
}

let new_fut () = { v = None; e = None; m = Mutex.create (); c = Condition.create () }

let resolve_ok fut x =
  Mutex.lock fut.m;
  fut.v <- Some x;
  Condition.broadcast fut.c;
  Mutex.unlock fut.m

let resolve_err fut ex =
  Mutex.lock fut.m;
  fut.e <- Some ex;
  Condition.broadcast fut.c;
  Mutex.unlock fut.m

let await fut =
  Mutex.lock fut.m;
  while fut.v = None && fut.e = None do Condition.wait fut.c fut.m done;
  let r = (fut.v, fut.e) in
  Mutex.unlock fut.m;
  match r with (Some v, _) -> v | (_, Some e) -> raise e | _ -> assert false

let spawn f =
  let fut = new_fut () in
  let _t = Thread.create (fun () ->
    (try resolve_ok fut (f ())
     with e -> resolve_err fut e)
  ) () in
  fut

(* join_all: wait for every future, collect results in order *)
let join_all futures =
  List.map await futures

(* join_all with error collection: returns list of Result *)
let join_all_results futures =
  List.map (fun fut ->
    try Ok (await fut)
    with e -> Error e
  ) futures

(* race: return the value of the first future that resolves *)
let race futures =
  let winner = new_fut () in
  List.iter (fun fut ->
    let _t = Thread.create (fun () ->
      (try resolve_ok winner (await fut)
       with _ -> ())
    ) () in ()
  ) futures;
  await winner

(* join_n: wait for first n out of m futures *)
let first_n n futures =
  let results = ref [] in
  let mutex = Mutex.create () in
  let cond = Condition.create () in
  List.iter (fun fut ->
    let _t = Thread.create (fun () ->
      let v = try Ok (await fut) with e -> Error e in
      Mutex.lock mutex;
      if List.length !results < n then begin
        results := v :: !results;
        if List.length !results = n then Condition.broadcast cond
      end;
      Mutex.unlock mutex
    ) () in ()
  ) futures;
  Mutex.lock mutex;
  while List.length !results < n do Condition.wait cond mutex done;
  let r = !results in
  Mutex.unlock mutex;
  List.rev r

(* Domain-based parallel join (CPU-bound) *)
let domain_join_all fns =
  let domains = List.map Domain.spawn fns in
  List.map Domain.join domains

let () =
  Printf.printf "=== join_all: wait for every task ===\n";
  let tasks = [
    (fun () -> Thread.delay 0.01; "task A");
    (fun () -> Thread.delay 0.005; "task B");
    (fun () -> Thread.delay 0.02; "task C");
  ] in
  let start = Unix.gettimeofday () in
  let futures = List.map spawn tasks in
  let results = join_all futures in
  let elapsed = Unix.gettimeofday () -. start in
  Printf.printf "results: [%s]\n" (String.concat "; " results);
  Printf.printf "elapsed: ~%.0fms (parallel, not serial ~35ms)\n" (elapsed *. 1000.0);

  Printf.printf "\n=== join_all_results: partial failure OK ===\n";
  let mixed = [
    spawn (fun () -> 42);
    spawn (fun () -> failwith "oops");
    spawn (fun () -> 99);
  ] in
  let res = join_all_results mixed in
  List.iter (function
    | Ok v   -> Printf.printf "  Ok %d\n" v
    | Error e -> Printf.printf "  Error: %s\n" (Printexc.to_string e)
  ) res;

  Printf.printf "\n=== race: first to finish wins ===\n";
  let racers = [
    spawn (fun () -> Thread.delay 0.02; "slow");
    spawn (fun () -> Thread.delay 0.005; "fast");
    spawn (fun () -> Thread.delay 0.01; "medium");
  ] in
  let winner = race racers in
  Printf.printf "winner: %s\n" winner;

  Printf.printf "\n=== Domain parallel join (CPU-bound) ===\n";
  let fib n =
    let rec f a b n = if n = 0 then a else f b (a+b) (n-1) in f 0 1 n
  in
  let results = domain_join_all [
    (fun () -> fib 30);
    (fun () -> fib 35);
    (fun () -> fib 25);
  ] in
  Printf.printf "fib results: [%s]\n"
    (String.concat "; " (List.map string_of_int results))
