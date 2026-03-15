(* 989: Once Initialization
   Run a function exactly once across all threads (lazy singleton pattern).
   OCaml: Lazy.t for single-threaded init; Mutex + flag for thread-safe once.
   OCaml 5 also has domain-safe Lazy via Domain-safe forcing. *)

(* --- Approach 1: OCaml's built-in Lazy.t --- *)
(* Lazy.t is evaluated at most once; subsequent force returns the cached value.
   In OCaml 5, Lazy.force is domain-safe for non-exception-raising thunks. *)
let lazy_config : (string * int) list Lazy.t =
  lazy begin
    Printf.printf "  [lazy] initializing config (runs once)...\n";
    [ ("host", 0); ("port", 8080); ("timeout", 30) ]
  end

(* --- Approach 2: Thread-safe Once cell (Mutex + flag) --- *)
type 'a once = {
  mutable value   : 'a option;
  mutable started : bool;
  mutex : Mutex.t;
  cond  : Condition.t;
}

let once_new () =
  { value = None; started = false; mutex = Mutex.create (); cond = Condition.create () }

(* Call f exactly once; all other callers block until init completes *)
let once_get_or_init cell f =
  Mutex.lock cell.mutex;
  (* Fast path: already initialized *)
  if cell.value <> None then begin
    let v = Option.get cell.value in
    Mutex.unlock cell.mutex;
    v
  end else if cell.started then begin
    (* Another thread is initializing — wait for it *)
    while cell.value = None do Condition.wait cell.cond cell.mutex done;
    let v = Option.get cell.value in
    Mutex.unlock cell.mutex;
    v
  end else begin
    cell.started <- true;
    Mutex.unlock cell.mutex;
    (* Initialize outside the lock to avoid holding it during slow init *)
    let v = f () in
    Mutex.lock cell.mutex;
    cell.value <- Some v;
    Condition.broadcast cell.cond;
    Mutex.unlock cell.mutex;
    v
  end

(* --- Approach 3: Atomic flag (spin-wait, for low-contention) --- *)
(* OCaml 5 provides Atomic for lock-free operations *)
type 'a atomic_once = {
  state : int Atomic.t;  (* 0=uninitialized, 1=initializing, 2=done *)
  mutable value : 'a option;
}

let atomic_once_new () = { state = Atomic.make 0; value = None }

let atomic_once_get_or_init ao f =
  match Atomic.get ao.state with
  | 2 -> Option.get ao.value   (* fast path *)
  | _ ->
    (* Try to become the initializer (CAS 0 → 1) *)
    if Atomic.compare_and_set ao.state 0 1 then begin
      let v = f () in
      ao.value <- Some v;
      Atomic.set ao.state 2;
      v
    end else begin
      (* Someone else is initializing; spin until done *)
      while Atomic.get ao.state < 2 do
        Domain.cpu_relax ()
      done;
      Option.get ao.value
    end

(* --- Singleton pattern using module-level Lazy --- *)
module Database = struct
  type t = { host : string; port : int; connections : int ref }

  let instance = lazy begin
    Printf.printf "  [Database] connecting (once)...\n";
    { host = "localhost"; port = 5432; connections = ref 0 }
  end

  let get () = Lazy.force instance

  let query db sql =
    db.connections := !(db.connections) + 1;
    Printf.sprintf "result(%s) conn#%d" sql !(db.connections)
end

let () =
  Printf.printf "=== Lazy.t (built-in once) ===\n";
  let v1 = Lazy.force lazy_config in
  let v2 = Lazy.force lazy_config in
  Printf.printf "port from v1=%d  same object=%b\n"
    (List.assoc "port" v1) (v1 == v2);   (* same physical object *)

  Printf.printf "\n=== Thread-safe Once cell ===\n";
  let cell : string once = once_new () in
  let results = ref [] in
  let mutex = Mutex.create () in

  (* 8 threads race to initialize *)
  let threads = List.init 8 (fun _ ->
    Thread.create (fun () ->
      let v = once_get_or_init cell (fun () ->
        Printf.printf "  [once] init called!\n";
        Thread.delay 0.01;
        "initialized-value"
      ) in
      Mutex.lock mutex;
      results := v :: !results;
      Mutex.unlock mutex
    ) ()
  ) in
  List.iter Thread.join threads;
  Printf.printf "all saw same value: %b\n"
    (List.for_all (fun v -> v = "initialized-value") !results);
  Printf.printf "init ran exactly once: (see above — one '[once] init called!')\n";

  Printf.printf "\n=== Atomic Once (OCaml 5 Atomic) ===\n";
  let ao : int atomic_once = atomic_once_new () in
  let domains = Array.init 4 (fun _ ->
    Domain.spawn (fun () ->
      atomic_once_get_or_init ao (fun () ->
        Printf.printf "  [atomic_once] init!\n";
        42)
    )
  ) in
  let values = Array.map Domain.join domains in
  Printf.printf "all got 42: %b\n" (Array.for_all (fun v -> v = 42) values);

  Printf.printf "\n=== Module singleton (Database) ===\n";
  let db1 = Database.get () in
  let db2 = Database.get () in
  Printf.printf "same instance: %b\n" (db1 == db2);
  Printf.printf "%s\n" (Database.query db1 "SELECT 1");
  Printf.printf "%s\n" (Database.query db2 "SELECT 2")
