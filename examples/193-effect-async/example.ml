(* Simulating async with effects — a cooperative scheduler *)
effect Yield : unit

let tasks = Queue.create ()

let async f = Queue.push f tasks

let run () =
  while not (Queue.is_empty tasks) do
    let task = Queue.pop tasks in
    match task () with
    | () -> ()
    | effect Yield k ->
      Queue.push (fun () -> continue k ()) tasks
  done

let () =
  async (fun () ->
    Printf.printf "Task A: step 1\n";
    perform Yield;
    Printf.printf "Task A: step 2\n");
  async (fun () ->
    Printf.printf "Task B: step 1\n";
    perform Yield;
    Printf.printf "Task B: step 2\n");
  run ()
