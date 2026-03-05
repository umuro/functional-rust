(* 446. Thread pool – OCaml *)
let make_pool n =
  let q = Queue.create () in
  let m = Mutex.create () in
  let c = Condition.create () in
  let stop = ref false in
  let workers = Array.init n (fun _ ->
    Thread.create (fun () ->
      let go = ref true in
      while !go do
        Mutex.lock m;
        while Queue.is_empty q && not !stop do Condition.wait c m done;
        if not (Queue.is_empty q) then
          let f = Queue.pop q in (Mutex.unlock m; f ())
        else (Mutex.unlock m; go := false)
      done) ()
  ) in
  let submit f = Mutex.lock m; Queue.push f q; Condition.signal c; Mutex.unlock m in
  let shutdown () =
    Mutex.lock m; stop := true; Condition.broadcast c; Mutex.unlock m;
    Array.iter Thread.join workers
  in
  (submit, shutdown)

let () =
  let (submit, shutdown) = make_pool 4 in
  for i = 1 to 8 do
    let n = i in submit (fun () -> Printf.printf "job %d\n%!" n)
  done;
  Thread.delay 0.05; shutdown ()
