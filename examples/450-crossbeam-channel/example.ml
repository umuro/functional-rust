(* 450. Bounded channel – OCaml manual *)
let make_bounded cap =
  let q = Queue.create () in
  let m = Mutex.create () in
  let nf = Condition.create () and ne = Condition.create () in
  let send v =
    Mutex.lock m;
    while Queue.length q >= cap do Condition.wait nf m done;
    Queue.push v q; Condition.signal ne; Mutex.unlock m in
  let recv () =
    Mutex.lock m;
    while Queue.is_empty q do Condition.wait ne m done;
    let v = Queue.pop q in Condition.signal nf; Mutex.unlock m; v
  in (send, recv)

let () =
  let (send, recv) = make_bounded 3 in
  let p = Thread.create (fun () ->
    for i=1 to 6 do send i; Printf.printf "sent %d\n%!" i done
  ) () in
  Thread.delay 0.02;
  for _ = 1 to 6 do Printf.printf "recv %d\n%!" (recv ()) done;
  Thread.join p
