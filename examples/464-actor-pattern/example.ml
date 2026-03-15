(* 464. Actor model – OCaml *)
type msg = Inc of int | Get of int Queue.t | Stop

let make_actor () =
  let q=Queue.create () let m=Mutex.create () let c=Condition.create () in
  let send v=Mutex.lock m; Queue.push v q; Condition.signal c; Mutex.unlock m in
  let recv ()=Mutex.lock m; while Queue.is_empty q do Condition.wait c m done;
    let v=Queue.pop q in Mutex.unlock m; v in
  ignore (Thread.create (fun () ->
    let st=ref 0 and go=ref true in
    while !go do match recv () with
    | Inc n -> st := !st+n
    | Get rq -> Queue.push !st rq
    | Stop -> go:=false
    done) ());
  send

let () =
  let send=make_actor () in
  send (Inc 10); send (Inc 5);
  let rq=Queue.create () in send (Get rq);
  (* spin wait *)
  while Queue.is_empty rq do Thread.delay 0.001 done;
  Printf.printf "state=%d\n" (Queue.pop rq);
  send Stop; Thread.delay 0.01
