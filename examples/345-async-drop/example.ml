(* 345: Async Drop / RAII cleanup
   OCaml uses finalizers and explicit cleanup functions.
   The idiomatic pattern is a "guard" value with a cleanup closure
   that runs when the guard goes out of scope (via GC finalizer)
   or is explicitly triggered. *)

(* Resource with a cleanup flag — analogous to Rust's Drop trait *)
type resource = {
  id         : int;
  cleaned_up : bool Atomic.t;
}

let make_resource id =
  let flag = Atomic.make false in
  let r = { id; cleaned_up = flag } in
  (* Register a finalizer — runs when GC collects the value *)
  Gc.finalise (fun r -> Atomic.set r.cleaned_up true) r;
  (r, flag)

(* Explicit cleanup (eager, deterministic — preferred over relying on GC) *)
let cleanup_resource r = Atomic.set r.cleaned_up true

(* Scope guard: runs cleanup when explicitly released *)
type 'a guard = {
  mutable cleanup : (unit -> unit) option;
  value : 'a;
}

let make_guard value cleanup = { cleanup = Some cleanup; value }

(* Disarm: cancel the cleanup (like Rust's ManuallyDrop) *)
let disarm g = g.cleanup <- None

(* Release: run cleanup now *)
let release g =
  match g.cleanup with
  | Some f -> g.cleanup <- None; f ()
  | None   -> ()

let () =
  (* Resource cleanup test *)
  let (r, flag) = make_resource 1 in
  assert (r.id = 1);
  assert (not (Atomic.get flag));
  cleanup_resource r;
  assert (Atomic.get flag);
  Printf.printf "Resource %d cleaned up: %b\n%!" r.id (Atomic.get flag);

  (* Guard test *)
  let called = Atomic.make false in
  let g = make_guard "data" (fun () -> Atomic.set called true) in
  assert (not (Atomic.get called));
  release g;
  assert (Atomic.get called);
  Printf.printf "Guard cleanup ran: %b\n%!" (Atomic.get called);

  (* Disarmed guard does NOT run cleanup *)
  let called2 = Atomic.make false in
  let g2 = make_guard "data2" (fun () -> Atomic.set called2 true) in
  disarm g2;
  release g2;
  assert (not (Atomic.get called2));
  Printf.printf "Disarmed guard skipped cleanup: %b\n%!" (not (Atomic.get called2))
