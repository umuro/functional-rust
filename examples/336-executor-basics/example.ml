(* OCaml: Lwt-based executor (simplified) *)

(* In Lwt, the executor is implicit:
   Lwt_main.run (
     let%lwt () = Lwt_io.printl "Task A" in
     let%lwt () = Lwt_io.printl "Task B" in
     Lwt.return ()
   )
*)

(* Sync simulation of task queue *)
let run_tasks tasks =
  let queue = Queue.create () in
  List.iter (fun t -> Queue.add t queue) tasks;
  while not (Queue.is_empty queue) do
    let task = Queue.pop queue in
    task ()
  done

let () =
  run_tasks [
    (fun () -> Printf.printf "Task A\n");
    (fun () -> Printf.printf "Task B\n");
    (fun () -> Printf.printf "Task C\n");
  ];
  Printf.printf "All done\n"
