(* 455. Lock-free stack – OCaml with Mutex (stdlib) *)
type 'a t = { mutable head: 'a list; m: Mutex.t }
let create () = { head=[]; m=Mutex.create () }
let push s v = Mutex.lock s.m; s.head <- v::s.head; Mutex.unlock s.m
let pop  s   = Mutex.lock s.m; let r = match s.head with []->None|x::t->s.head<-t;Some x in Mutex.unlock s.m; r
let () =
  let s = create () in
  let ps = List.init 4 (fun id ->
    Thread.create (fun () -> for i=0 to 9 do push s (id*10+i) done) ()
  ) in
  List.iter Thread.join ps;
  let n = ref 0 in while Option.is_some (pop s) do incr n done;
  Printf.printf "popped %d\n" !n
