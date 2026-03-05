(* 443. Arc<Mutex<T>> – OCaml *)
let counter = ref 0
let mutex   = Mutex.create ()

let () =
  let threads = List.init 10 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 100 do
        Mutex.lock mutex; incr counter; Mutex.unlock mutex
      done) ()
  ) in
  List.iter Thread.join threads;
  Printf.printf "Counter = %d (expected 1000)\n" !counter
