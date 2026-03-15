(* 986: Mutex Basics
   Mutual exclusion: only one thread accesses a shared resource at a time.
   OCaml: Mutex.t — same semantics as Rust's std::sync::Mutex.
   Key difference: OCaml's GC means there's no "poisoning" on panic. *)

(* --- Protected counter --- *)
type counter = {
  mutable value : int;
  mutex : Mutex.t;
}

let make_counter () = { value = 0; mutex = Mutex.create () }

let increment c =
  Mutex.lock c.mutex;
  c.value <- c.value + 1;
  Mutex.unlock c.mutex

(* RAII-style: run f while holding the lock; always unlock even on exception *)
let with_lock mutex f =
  Mutex.lock mutex;
  match f () with
  | v -> Mutex.unlock mutex; v
  | exception e -> Mutex.unlock mutex; raise e

let get_value c = with_lock c.mutex (fun () -> c.value)

(* --- Protected map (shared mutable dictionary) --- *)
type ('k, 'v) protected_map = {
  table : ('k, 'v) Hashtbl.t;
  mutex : Mutex.t;
}

let make_map () = { table = Hashtbl.create 16; mutex = Mutex.create () }

let map_insert pm k v =
  with_lock pm.mutex (fun () -> Hashtbl.replace pm.table k v)

let map_find pm k =
  with_lock pm.mutex (fun () -> Hashtbl.find_opt pm.table k)

let map_update pm k f default =
  with_lock pm.mutex (fun () ->
    let v = Option.value (Hashtbl.find_opt pm.table k) ~default in
    Hashtbl.replace pm.table k (f v))

(* --- Try-lock pattern --- *)
let try_with_lock mutex f =
  if Mutex.try_lock mutex then begin
    match f () with
    | v -> Mutex.unlock mutex; Some v
    | exception e -> Mutex.unlock mutex; raise e
  end else None

(* --- Demonstrate data races without mutex (educational) --- *)
(* NOTE: this is intentionally racy to show why mutex is needed *)
let racy_increment shared n_threads n_iters =
  let threads = List.init n_threads (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to n_iters do
        shared := !shared + 1  (* race condition *)
      done
    ) ()
  ) in
  List.iter Thread.join threads

let () =
  Printf.printf "=== Protected counter ===\n";
  let counter = make_counter () in
  let n_threads = 8 and n_iters = 1000 in
  let threads = List.init n_threads (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to n_iters do increment counter done
    ) ()
  ) in
  List.iter Thread.join threads;
  let expected = n_threads * n_iters in
  Printf.printf "counter = %d (expected %d, correct = %b)\n"
    (get_value counter) expected (get_value counter = expected);

  Printf.printf "\n=== with_lock (RAII pattern) ===\n";
  let mutex = Mutex.create () in
  let shared = ref 0 in
  let workers = List.init 4 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 500 do
        with_lock mutex (fun () -> shared := !shared + 1)
      done
    ) ()
  ) in
  List.iter Thread.join workers;
  Printf.printf "shared = %d (expected 2000)\n" !shared;

  Printf.printf "\n=== Protected map (word frequency count) ===\n";
  let pm : (string, int) protected_map = make_map () in
  let words_per_thread = ["the"; "quick"; "brown"; "fox"; "the"; "lazy"; "the"] in
  let ts = List.init 4 (fun _ ->
    Thread.create (fun () ->
      List.iter (fun w ->
        map_update pm w (fun c -> c + 1) 0
      ) words_per_thread
    ) ()
  ) in
  List.iter Thread.join ts;
  let the_count = Option.value (map_find pm "the") ~default:0 in
  Printf.printf "\"the\" count = %d (expected %d)\n"
    the_count (4 * 3);  (* 4 threads × 3 occurrences *)

  Printf.printf "\n=== try_lock (non-blocking) ===\n";
  let busy_mutex = Mutex.create () in
  Mutex.lock busy_mutex;
  let result = try_with_lock busy_mutex (fun () -> "got the lock") in
  Printf.printf "try_lock on held mutex: %s\n"
    (Option.value result ~default:"None (already locked)");
  Mutex.unlock busy_mutex;
  let result2 = try_with_lock busy_mutex (fun () -> "got the lock") in
  Printf.printf "try_lock on free mutex: %s\n"
    (Option.value result2 ~default:"None")
