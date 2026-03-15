(* OCaml: oneshot via Mutex + flag *)

type 'a oneshot = {
  mutable value : 'a option;
  mutex : Mutex.t;
  cond  : Condition.t;
}

let make () =
  { value = None; mutex = Mutex.create (); cond = Condition.create () }

let send os v =
  Mutex.lock os.mutex;
  os.value <- Some v;
  Condition.signal os.cond;
  Mutex.unlock os.mutex

let recv os =
  Mutex.lock os.mutex;
  while os.value = None do
    Condition.wait os.cond os.mutex
  done;
  let v = Option.get os.value in
  Mutex.unlock os.mutex;
  v

let () =
  let os = make () in
  let _ = Thread.create (fun () ->
    Thread.delay 0.01;
    send os 42
  ) () in
  let v = recv os in
  Printf.printf "Received: %d\n" v
