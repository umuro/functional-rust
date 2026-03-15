(* 993: Thread Pool / Work Queue *)
(* Fixed N workers consuming from a shared channel *)

type 'a chan = { q: 'a Queue.t; m: Mutex.t; cond: Condition.t; mutable closed: bool }

let make_chan () = { q = Queue.create (); m = Mutex.create ();
                     cond = Condition.create (); closed = false }

let send c v =
  Mutex.lock c.m;
  Queue.push v c.q;
  Condition.signal c.cond;
  Mutex.unlock c.m

let close_chan c =
  Mutex.lock c.m;
  c.closed <- true;
  Condition.broadcast c.cond;
  Mutex.unlock c.m

let recv_work c =
  Mutex.lock c.m;
  while Queue.is_empty c.q && not c.closed do
    Condition.wait c.cond c.m
  done;
  let v = if Queue.is_empty c.q then None else Some (Queue.pop c.q) in
  Mutex.unlock c.m;
  v

(* --- Thread pool: spawn N workers, each pulls from shared queue --- *)

type 'a pool = {
  work_chan: ('a -> unit) chan;
  workers: Thread.t list;
}

let make_pool n =
  let work_chan = make_chan () in
  let workers = List.init n (fun _ ->
    Thread.create (fun () ->
      let rec loop () =
        match recv_work work_chan with
        | None -> ()  (* channel closed, exit *)
        | Some task -> task (); loop ()
      in
      loop ()
    ) ()
  ) in
  { work_chan; workers }

let submit pool task = send pool.work_chan task

let shutdown pool =
  close_chan pool.work_chan;
  List.iter Thread.join pool.workers

(* --- Approach 1: Submit 20 tasks to a pool of 4 workers --- *)

let () =
  let results = ref [] in
  let m = Mutex.create () in
  let pool = make_pool 4 in

  for i = 1 to 20 do
    let i = i in
    submit pool (fun () ->
      let v = i * i in
      Mutex.lock m;
      results := v :: !results;
      Mutex.unlock m
    )
  done;

  shutdown pool;

  let sorted = List.sort compare !results in
  (* 1^2..20^2 = 1,4,9,...,400 — sum = 2870 *)
  let total = List.fold_left (+) 0 sorted in
  assert (List.length sorted = 20);
  assert (total = 2870);
  Printf.printf "Approach 1 (pool of 4, 20 tasks): sum=%d\n" total

let () = Printf.printf "✓ All tests passed\n"
