(* 468. Lock-free queue concept – OCaml with Mutex *)
type 'a node = { v: 'a option; mutable next: 'a node option }
type 'a q = { mutable head: 'a node; mutable tail: 'a node; m: Mutex.t }

let create () =
  let dummy={v=None;next=None} in
  {head=dummy;tail=dummy;m=Mutex.create ()}

let enqueue q v =
  let n={v=Some v;next=None} in
  Mutex.lock q.m; q.tail.next<-Some n; q.tail<-n; Mutex.unlock q.m

let dequeue q =
  Mutex.lock q.m;
  let r = match q.head.next with
    | None -> None
    | Some n -> q.head<-n; n.v
  in Mutex.unlock q.m; r

let () =
  let q=create () in
  List.iter (enqueue q) [1;2;3;4;5];
  let rec drain () = match dequeue q with None->() | Some v->Printf.printf "%d " v; drain ()
  in drain (); print_newline ()
