(* 467: Epoch-Based Garbage Collection
   A simplified epoch manager: domains "pin" the current epoch while
   reading; the GC thread advances the epoch and frees objects that
   are older than the minimum pinned epoch. *)

type epoch_mgr = {
  epoch   : int Atomic.t;
  mu      : Mutex.t;
  retired : (int * string) Queue.t;  (* (epoch_retired, description) *)
  pinned  : int list ref;            (* epochs currently pinned by readers *)
}

let make () = {
  epoch   = Atomic.make 0;
  mu      = Mutex.create ();
  retired = Queue.create ();
  pinned  = ref [];
}

(* Pin the current epoch; returns the pinned epoch number *)
let pin mgr =
  let e = Atomic.get mgr.epoch in
  Mutex.lock mgr.mu;
  mgr.pinned := e :: !(mgr.pinned);
  Mutex.unlock mgr.mu;
  e

(* Unpin an epoch (reader done) *)
let unpin mgr e =
  Mutex.lock mgr.mu;
  mgr.pinned := (List.filter (fun x -> x <> e) !(mgr.pinned));
  Mutex.unlock mgr.mu

let retire mgr desc =
  let e = Atomic.get mgr.epoch in
  Mutex.lock mgr.mu;
  Queue.push (e, desc) mgr.retired;
  Mutex.unlock mgr.mu

(* Advance epoch; free objects whose epoch < min-pinned - 1 *)
let collect mgr =
  let new_e = Atomic.fetch_and_add mgr.epoch 1 + 1 in
  Mutex.lock mgr.mu;
  let min_active =
    match !(mgr.pinned) with
    | [] -> new_e
    | ps -> List.fold_left min new_e ps
  in
  let safe_before = max 0 (min_active - 1) in
  let freed = ref 0 in
  (* Drain all items that are safe to free *)
  let remaining = Queue.create () in
  Queue.iter (fun (e, d) ->
    if e <= safe_before then begin
      Printf.printf "  freed: %s (epoch %d)\n%!" d e;
      incr freed
    end else
      Queue.push (e, d) remaining
  ) mgr.retired;
  Queue.clear mgr.retired;
  Queue.iter (fun x -> Queue.push x mgr.retired) remaining;
  Mutex.unlock mgr.mu;
  Printf.printf "epoch→%d; freed %d; deferred %d\n%!"
    new_e !freed (Queue.length mgr.retired)

let () =
  let m = make () in
  collect m;  (* epoch 0 → 1 *)
  assert (Atomic.get m.epoch = 1);
  Printf.printf "epoch after first collect: %d\n%!" (Atomic.get m.epoch);

  retire m "object-A";
  assert (Queue.length m.retired = 1);
  Printf.printf "retired 1 object\n%!";

  (* Pin keeps object alive; collect advances epoch but cannot free it yet *)
  let pinned_e = pin m in
  collect m;  (* epoch 1 → 2; pinned_e=1 so safe_before = 0; nothing freed *)
  unpin m pinned_e;

  (* Now no pins; next collect can free object-A *)
  collect m;  (* epoch 2 → 3; min_active=3, safe_before=2; object-A (epoch 1) freed *)
  assert (Queue.length m.retired = 0);
  Printf.printf "all objects freed\n%!"
