(* 980: Async Map
   Apply a function concurrently to each element of a collection.
   OCaml: Thread pool + futures, or Domain.spawn for CPU-bound parallel map.
   Demonstrates both concurrent (I/O-bound) and parallel (CPU-bound) map. *)

(* --- Reuse the future primitives from 979 --- *)
type 'a future = {
  mutable value : 'a option;
  mutable exn   : exn option;
  mutex : Mutex.t;
  cond  : Condition.t;
}

let make_future () =
  { value = None; exn = None; mutex = Mutex.create (); cond = Condition.create () }

let resolve_future fut v =
  Mutex.lock fut.mutex;
  fut.value <- Some v;
  Condition.broadcast fut.cond;
  Mutex.unlock fut.mutex

let await_future fut =
  Mutex.lock fut.mutex;
  while fut.value = None && fut.exn = None do
    Condition.wait fut.cond fut.mutex
  done;
  let r = fut.value in
  Mutex.unlock fut.mutex;
  match r with Some v -> v | None -> raise (Option.get fut.exn)

(* --- Concurrent map: each element processed in its own thread --- *)
let async_map f lst =
  let futures = List.map (fun x ->
    let fut = make_future () in
    let _t = Thread.create (fun () ->
      (try resolve_future fut (f x)
       with e ->
         Mutex.lock fut.mutex;
         fut.exn <- Some e;
         Condition.broadcast fut.cond;
         Mutex.unlock fut.mutex)
    ) () in
    fut
  ) lst in
  List.map await_future futures

(* --- Parallel map using OCaml 5 Domains (true CPU parallelism) --- *)
let parallel_map f arr =
  let n = Array.length arr in
  let domains = Array.init n (fun i ->
    Domain.spawn (fun () -> f arr.(i))
  ) in
  Array.map Domain.join domains

(* --- Bounded concurrency map (thread pool of size k) --- *)
let pool_map ~workers f lst =
  let input  = Queue.of_seq (List.to_seq (List.mapi (fun i x -> (i,x)) lst)) in
  let output = Array.make (List.length lst) (Obj.magic ()) in
  let mutex  = Mutex.create () in
  let done_  = ref 0 in
  let total  = List.length lst in

  let worker () =
    let continue_ = ref true in
    while !continue_ do
      Mutex.lock mutex;
      if Queue.is_empty input then (continue_ := false; Mutex.unlock mutex)
      else begin
        let (i, x) = Queue.pop input in
        Mutex.unlock mutex;
        let result = f x in
        Mutex.lock mutex;
        output.(i) <- result;
        incr done_;
        Mutex.unlock mutex
      end
    done
  in

  let threads = List.init workers (fun _ -> Thread.create worker ()) in
  List.iter Thread.join threads;
  assert (!done_ = total);
  Array.to_list output

let () =
  (* --- Concurrent I/O-bound map: fetch "data" for each id --- *)
  Printf.printf "=== Async map (concurrent) ===\n";
  let fetch_data id =
    Thread.delay (0.01 *. float_of_int (1 + id mod 3));
    Printf.sprintf "data_for_%d" id
  in
  let ids = [1; 2; 3; 4; 5] in
  let start = Unix.gettimeofday () in
  let results = async_map fetch_data ids in
  let elapsed = Unix.gettimeofday () -. start in
  Printf.printf "results: [%s]\n" (String.concat "; " results);
  Printf.printf "elapsed: %.0fms (concurrent, not serial)\n" (elapsed *. 1000.0);

  (* --- Parallel CPU-bound map --- *)
  Printf.printf "\n=== Parallel map (Domains, CPU-bound) ===\n";
  let arr = [|1; 2; 3; 4; 5; 6; 7; 8|] in
  let squared = parallel_map (fun x -> x * x) arr in
  Printf.printf "squared: [%s]\n"
    (String.concat "; " (Array.to_list (Array.map string_of_int squared)));

  (* --- Pool map with bounded concurrency --- *)
  Printf.printf "\n=== Pool map (4 workers) ===\n";
  let items = List.init 12 (fun i -> i + 1) in
  let processed = pool_map ~workers:4 (fun x -> x * x + 1) items in
  Printf.printf "pool results: [%s]\n"
    (String.concat "; " (List.map string_of_int processed));

  (* --- filter_map style: keep only successful results --- *)
  Printf.printf "\n=== Async filter_map ===\n";
  let parse_int s =
    try Some (int_of_string s)
    with _ -> None
  in
  let strings = ["1"; "two"; "3"; "four"; "5"] in
  let parsed = async_map parse_int strings in
  let valid = List.filter_map (fun x -> x) parsed in
  Printf.printf "parsed ints: [%s]\n"
    (String.concat "; " (List.map string_of_int valid))
