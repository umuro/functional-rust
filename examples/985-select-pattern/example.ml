(* 985: Select Pattern — Poll Multiple Channels *)
(* OCaml: Lwt.pick / try_receive with non-blocking checks *)

type 'a chan = {
  q: 'a Queue.t;
  m: Mutex.t;
  cond: Condition.t;
  mutable closed: bool;
}

let make_chan () = { q = Queue.create (); m = Mutex.create ();
                     cond = Condition.create (); closed = false }

let send c v =
  Mutex.lock c.m;
  Queue.push v c.q;
  Condition.signal c.cond;
  Mutex.unlock c.m

let try_recv c =
  Mutex.lock c.m;
  let v = if Queue.is_empty c.q then None else Some (Queue.pop c.q) in
  Mutex.unlock c.m;
  v

let close_chan c =
  Mutex.lock c.m;
  c.closed <- true;
  Condition.broadcast c.cond;
  Mutex.unlock c.m

let is_closed c =
  Mutex.lock c.m;
  let v = c.closed && Queue.is_empty c.q in
  Mutex.unlock c.m;
  v

(* --- Approach 1: Non-blocking select loop over two channels --- *)

type ('a, 'b) select_result = Left of 'a | Right of 'b | Both_closed

let select c1 c2 =
  let rec loop () =
    match try_recv c1, try_recv c2 with
    | Some v, _ -> Left v
    | None, Some v -> Right v
    | None, None ->
      if is_closed c1 && is_closed c2 then Both_closed
      else (Thread.yield (); loop ())
  in
  loop ()

let () =
  let c1 = make_chan () in
  let c2 = make_chan () in

  (* Producer for c1 *)
  let p1 = Thread.create (fun () ->
    List.iter (fun i -> send c1 i; Unix.sleepf 0.001) [1;2;3];
    close_chan c1
  ) () in

  (* Producer for c2 *)
  let p2 = Thread.create (fun () ->
    List.iter (fun s -> send c2 s; Unix.sleepf 0.001) ["a";"b";"c"];
    close_chan c2
  ) () in

  let lefts = ref [] and rights = ref [] in
  let rec drain () =
    match select c1 c2 with
    | Left v -> lefts := v :: !lefts; drain ()
    | Right v -> rights := v :: !rights; drain ()
    | Both_closed -> ()
  in
  drain ();
  Thread.join p1; Thread.join p2;

  let lefts = List.sort compare !lefts in
  let rights = List.sort compare !rights in
  assert (lefts = [1;2;3]);
  assert (rights = ["a";"b";"c"]);
  Printf.printf "Approach 1 (select): lefts=[%s] rights=[%s]\n"
    (String.concat ";" (List.map string_of_int lefts))
    (String.concat ";" rights)

let () = Printf.printf "✓ All tests passed\n"
