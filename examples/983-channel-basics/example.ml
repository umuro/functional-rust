(* 983: MPSC Channel Basics *)
(* OCaml: Thread + Event module for synchronous channels *)

(* --- Approach 1: Simple Thread + Mutex queue (simulates MPSC) --- *)

let () =
  let q = Queue.create () in
  let m = Mutex.create () in
  let cond = Condition.create () in
  let done_ = ref false in

  (* Producer thread *)
  let producer = Thread.create (fun () ->
    List.iter (fun msg ->
      Mutex.lock m;
      Queue.push msg q;
      Condition.signal cond;
      Mutex.unlock m
    ) [1; 2; 3; 4; 5];
    Mutex.lock m;
    done_ := true;
    Condition.signal cond;
    Mutex.unlock m
  ) () in

  (* Consumer (main thread) *)
  let results = ref [] in
  let running = ref true in
  while !running do
    Mutex.lock m;
    while Queue.is_empty q && not !done_ do
      Condition.wait cond m
    done;
    if Queue.is_empty q && !done_ then
      running := false
    else if not (Queue.is_empty q) then
      results := Queue.pop q :: !results;
    Mutex.unlock m
  done;
  Thread.join producer;
  let results = List.sort compare !results in
  assert (results = [1;2;3;4;5]);
  Printf.printf "Approach 1 (Thread+Queue): [%s]\n"
    (String.concat "; " (List.map string_of_int results))

(* --- Approach 2: Multiple producers, one consumer --- *)

let () =
  let q = Queue.create () in
  let m = Mutex.create () in
  let cond = Condition.create () in
  let pending = ref 3 in (* 3 producers *)

  let make_producer start =
    Thread.create (fun () ->
      for i = start to start + 2 do
        Mutex.lock m;
        Queue.push i q;
        Condition.signal cond;
        Mutex.unlock m
      done;
      Mutex.lock m;
      decr pending;
      Condition.signal cond;
      Mutex.unlock m
    ) ()
  in

  let p1 = make_producer 1 in
  let p2 = make_producer 10 in
  let p3 = make_producer 100 in

  let results = ref [] in
  let running = ref true in
  while !running do
    Mutex.lock m;
    while Queue.is_empty q && !pending > 0 do
      Condition.wait cond m
    done;
    if not (Queue.is_empty q) then
      results := Queue.pop q :: !results
    else if !pending = 0 then
      running := false;
    Mutex.unlock m
  done;
  List.iter Thread.join [p1; p2; p3];
  let results = List.sort compare !results in
  assert (List.length results = 9);
  Printf.printf "Approach 2 (multi-producer): %d items\n" (List.length results)

let () = Printf.printf "✓ All tests passed\n"
