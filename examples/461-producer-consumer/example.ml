(* 461: Producer-Consumer with multiple producers
   Two producer domains feed a shared channel; one consumer
   domain drains it. Uses a Mutex-protected Queue as the channel. *)

type 'a chan = {
  q    : 'a Queue.t;
  mu   : Mutex.t;
  cond : Condition.t;
  (* None = still open; Some = producers closed *)
  done_flag : bool ref;
}

let make_chan () = {
  q = Queue.create (); mu = Mutex.create ();
  cond = Condition.create (); done_flag = ref false;
}

let send ch v =
  Mutex.lock ch.mu;
  Queue.push v ch.q;
  Condition.signal ch.cond;
  Mutex.unlock ch.mu

let close ch =
  Mutex.lock ch.mu;
  ch.done_flag := true;
  Condition.broadcast ch.cond;
  Mutex.unlock ch.mu

(* Drain the channel until closed AND empty *)
let recv_all ch =
  let results = ref [] in
  Mutex.lock ch.mu;
  let running = ref true in
  while !running do
    while Queue.is_empty ch.q && not !(ch.done_flag) do
      Condition.wait ch.cond ch.mu
    done;
    while not (Queue.is_empty ch.q) do
      results := Queue.pop ch.q :: !results
    done;
    if !(ch.done_flag) then running := false
  done;
  Mutex.unlock ch.mu;
  List.rev !results

let () =
  let ch = make_chan () in

  (* 2 producers, each sends 5 items *)
  let producers = List.init 2 (fun id ->
    Domain.spawn (fun () ->
      for i = 0 to 4 do send ch (id * 10 + i) done))
  in

  (* Close after producers finish *)
  let closer = Domain.spawn (fun () ->
    List.iter Domain.join producers;
    close ch)
  in

  let items = recv_all ch in
  Domain.join closer;

  assert (List.length items = 10);
  Printf.printf "consumed %d items\n%!" (List.length items);
  let sorted = List.sort compare items in
  Printf.printf "sorted: %s\n%!"
    (sorted |> List.map string_of_int |> String.concat ", ")
