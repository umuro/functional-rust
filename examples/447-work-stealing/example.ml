(* 447. Work-stealing – OCaml *)
(* Simplified shared queue; real WS needs per-thread deques *)
let deque = ref (List.init 20 (fun i -> i+1))
let mutex  = Mutex.create ()

let steal () =
  Mutex.lock mutex;
  let r = match !deque with [] -> None | x::rest -> deque:=rest; Some x in
  Mutex.unlock mutex; r

let () =
  let done_ = ref 0 in let dm = Mutex.create () in
  let workers = Array.init 4 (fun id ->
    Thread.create (fun () ->
      let rec loop () =
        match steal () with
        | None -> ()
        | Some j ->
          Printf.printf "worker %d: job %d\n%!" id j;
          Mutex.lock dm; incr done_; Mutex.unlock dm;
          loop ()
      in loop ()
    ) ()
  ) in
  Array.iter Thread.join workers;
  Printf.printf "done: %d\n" !done_
