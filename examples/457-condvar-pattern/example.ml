(* 457. Condvar – OCaml *)
let m=Mutex.create () let c=Condition.create () let items=Queue.create ()
let done_=ref false

let () =
  let prod = Thread.create (fun () ->
    for i=1 to 5 do
      Thread.delay 0.01;
      Mutex.lock m; Queue.push i items; Condition.signal c; Mutex.unlock m
    done;
    Mutex.lock m; done_:=true; Condition.broadcast c; Mutex.unlock m
  ) () in
  let cons = Thread.create (fun () ->
    let go = ref true in
    while !go do
      Mutex.lock m;
      while Queue.is_empty items && not !done_ do Condition.wait c m done;
      if not (Queue.is_empty items) then
        let v = Queue.pop items in (Mutex.unlock m; Printf.printf "got %d\n%!" v)
      else (Mutex.unlock m; go:=false)
    done
  ) () in
  Thread.join prod; Thread.join cons
