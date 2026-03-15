(* 461. Producer-consumer – OCaml *)
let q=Queue.create () let m=Mutex.create ()
let ne=Condition.create () let nf=Condition.create ()
let cap=5 let done_=ref false

let produce v = Mutex.lock m; while Queue.length q>=cap do Condition.wait nf m done;
  Queue.push v q; Condition.signal ne; Mutex.unlock m

let consume () = Mutex.lock m; while Queue.is_empty q && not !done_ do Condition.wait ne m done;
  let r = if Queue.is_empty q then None
          else (let v=Queue.pop q in Condition.signal nf; Some v) in
  Mutex.unlock m; r

let () =
  let p=Thread.create (fun () ->
    for i=1 to 10 do produce i done;
    Mutex.lock m; done_:=true; Condition.broadcast ne; Mutex.unlock m
  ) () in
  let rec loop () = match consume () with None->() | Some v -> Printf.printf "got %d\n%!" v; loop () in
  Thread.create loop () |> Thread.join;
  Thread.join p
