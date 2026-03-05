(* 445. MPSC channels – OCaml Queue+Mutex+Condition *)
let q=Queue.create () let m=Mutex.create () let c=Condition.create ()
let send v = Mutex.lock m; Queue.push v q; Condition.signal c; Mutex.unlock m
let recv () = Mutex.lock m; while Queue.is_empty q do Condition.wait c m done;
  let v=Queue.pop q in Mutex.unlock m; v

let () =
  let producers = List.init 3 (fun id ->
    Thread.create (fun () ->
      for i=1 to 5 do send (Printf.sprintf "p%d-msg%d" id i) done
    ) ()
  ) in
  let consumer = Thread.create (fun () ->
    for _ = 1 to 15 do Printf.printf "got: %s\n%!" (recv ()) done
  ) () in
  List.iter Thread.join producers; Thread.join consumer
