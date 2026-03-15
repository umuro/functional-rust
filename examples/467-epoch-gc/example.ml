(* 467. Epoch GC concept – OCaml *)
(* OCaml has a GC; this demonstrates the concept of deferred reclamation *)

type 'a retired = { value: 'a; epoch: int }

let global_epoch = ref 0
let retired_list : 'a retired list ref = ref []
let mutex = Mutex.create ()

let retire v =
  Mutex.lock mutex;
  let e = !global_epoch in
  retired_list := { value=v; epoch=e } :: !retired_list;
  Mutex.unlock mutex

let collect min_safe_epoch =
  Mutex.lock mutex;
  let (safe, keep) = List.partition (fun r -> r.epoch < min_safe_epoch) !retired_list in
  retired_list := keep;
  Mutex.unlock mutex;
  Printf.printf "reclaiming %d objects\n" (List.length safe)

let advance () =
  Mutex.lock mutex; incr global_epoch; Mutex.unlock mutex

let () =
  retire "node-A"; retire "node-B"; retire "node-C";
  Printf.printf "retired 3, epoch=%d\n" !global_epoch;
  advance ();
  Printf.printf "advanced to epoch=%d\n" !global_epoch;
  collect 1;  (* safe epoch: reclaim anything from epoch < 1 *)
  Printf.printf "remaining: %d\n" (List.length !retired_list)
