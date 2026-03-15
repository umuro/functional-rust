(* 456: Once-Cell / Lazy Initialization
   OCaml has Lazy.t for single-threaded lazy values.
   For thread-safe, initialize-once semantics (OnceLock), we use
   a Mutex + option ref, or the stdlib's Lazy module which is
   thread-safe in OCaml 5. *)

(* Thread-safe lazy value — equivalent to Rust's OnceLock<T> *)
type 'a once_lock = {
  mutex : Mutex.t;
  value : 'a option ref;
}

let make_once () = { mutex = Mutex.create (); value = ref None }

(* Initialize at most once; subsequent calls return the cached value *)
let get_or_init lock f =
  (* Fast path: already initialised *)
  match !(lock.value) with
  | Some v -> v
  | None ->
    Mutex.lock lock.mutex;
    (* Double-check inside the lock *)
    let v = match !(lock.value) with
      | Some v -> v
      | None   ->
        let v = f () in
        lock.value := Some v;
        v
    in
    Mutex.unlock lock.mutex;
    v

let get lock = !(lock.value)

(* OCaml's built-in Lazy.t is also safe for concurrent force in OCaml 5 *)
let config = lazy (
  Printf.printf "(init config)\n%!";
  [("host", "localhost"); ("port", "8080")]
)

let () =
  (* OnceLock: init function called exactly once *)
  let lock = make_once () in
  let call_count = Atomic.make 0 in
  let v1 = get_or_init lock (fun () ->
    ignore (Atomic.fetch_and_add call_count 1); 42) in
  let v2 = get_or_init lock (fun () ->
    ignore (Atomic.fetch_and_add call_count 1); 99) in
  assert (v1 = 42);
  assert (v2 = 42);  (* not re-initialized *)
  assert (Atomic.get call_count = 1);
  Printf.printf "once_lock: v=%d calls=%d\n%!" v1 (Atomic.get call_count);

  (* get before init returns None *)
  let lock2 = make_once () in
  assert (get lock2 = None);
  Printf.printf "uninitialized get: None\n%!";

  (* Lazy.t — idiomatic OCaml for module-level lazy init *)
  let cfg = Lazy.force config in
  let cfg2 = Lazy.force config in  (* does not print again *)
  assert (cfg = cfg2);
  Printf.printf "lazy config host=%s\n%!" (List.assoc "host" cfg)
