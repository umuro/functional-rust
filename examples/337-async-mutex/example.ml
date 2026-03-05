(* OCaml: Mutex for thread synchronization *)

let sync_mutex_demo () =
  let counter = ref 0 in
  let m = Mutex.create () in
  let threads = List.init 10 (fun _ ->
    Thread.create (fun () ->
      Mutex.lock m;
      incr counter;
      Mutex.unlock m
    ) ()
  ) in
  List.iter Thread.join threads;
  !counter

let () =
  Printf.printf "Counter: %d\n" (sync_mutex_demo ())
