(* OCaml: structured concurrency via nested thread management *)

let run_scoped tasks =
  let handles = List.map (fun f -> Thread.create f ()) tasks in
  List.iter Thread.join handles

let with_resources setup teardown f =
  let r = setup () in
  (try f r with e -> teardown r; raise e);
  teardown r

let () =
  run_scoped [
    (fun () ->
      Thread.delay 0.01;
      Printf.printf "Task A done\n");
    (fun () ->
      Thread.delay 0.005;
      Printf.printf "Task B done\n");
    (fun () ->
      Printf.printf "Task C done\n");
  ];
  Printf.printf "All tasks in scope completed\n"
