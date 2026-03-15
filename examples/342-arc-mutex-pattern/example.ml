(* 342: Arc<Mutex<T>> Pattern — Thread-safe shared mutable state.
   OCaml: Mutex for exclusive access; shared state via closures or records.
   OCaml 5 adds Domain for true parallelism; Thread works for concurrency. *)

(* Shared counter: multiple threads increment the same counter *)
let shared_counter num_threads =
  let mutex = Mutex.create () in
  let counter = ref 0 in
  let threads = Array.init num_threads (fun _ ->
    Thread.create (fun () ->
      Mutex.lock mutex;
      incr counter;
      Mutex.unlock mutex
    ) ()
  ) in
  Array.iter Thread.join threads;
  !counter

(* Thread-safe cache backed by a Mutex-protected list *)
type 'a thread_safe_cache = {
  mutable data : 'a list;
  mutex : Mutex.t;
}

let make_cache () = { data = []; mutex = Mutex.create () }

let cache_push cache item =
  Mutex.lock cache.mutex;
  cache.data <- item :: cache.data;
  Mutex.unlock cache.mutex

let cache_get_all cache =
  Mutex.lock cache.mutex;
  let snapshot = cache.data in
  Mutex.unlock cache.mutex;
  List.rev snapshot   (* return in insertion order *)

let cache_len cache =
  Mutex.lock cache.mutex;
  let n = List.length cache.data in
  Mutex.unlock cache.mutex;
  n

(* ── Domain-based parallel counter (OCaml 5 multicore) ─────────────────────
   Domain.spawn runs on a separate OS thread with true parallelism.
   Atomic provides lock-free increment on OCaml 5. *)
let parallel_counter_atomic n =
  let counter = Atomic.make 0 in
  let domains = Array.init n (fun _ ->
    Domain.spawn (fun () -> Atomic.fetch_and_add counter 1 |> ignore)
  ) in
  Array.iter Domain.join domains;
  Atomic.get counter

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* Thread-safe counter *)
  let count = shared_counter 10 in
  Printf.printf "shared_counter(10) = %d\n" count;
  assert (count = 10);

  (* Thread-safe cache *)
  let cache = make_cache () in
  let threads = Array.init 5 (fun i ->
    Thread.create (fun () -> cache_push cache i) ()
  ) in
  Array.iter Thread.join threads;
  Printf.printf "cache len after 5 threads = %d\n" (cache_len cache);
  assert (cache_len cache = 5);

  (* All values present *)
  let values = cache_get_all cache |> List.sort compare in
  Printf.printf "cache values = [%s]\n"
    (values |> List.map string_of_int |> String.concat ";");

  (* OCaml 5 Domain + Atomic *)
  let parallel_count = parallel_counter_atomic 8 in
  Printf.printf "parallel_counter_atomic(8) = %d\n" parallel_count;
  assert (parallel_count = 8)
