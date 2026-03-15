(* OCaml: cooperative cancellation via flag *)

let cancelled = ref false

let cancel () = cancelled := true

let long_task steps =
  let rec loop i =
    if !cancelled then
      Printf.printf "Task cancelled at step %d\n" i
    else if i >= steps then
      Printf.printf "Task completed all %d steps\n" steps
    else begin
      Printf.printf "Step %d...\n" i;
      Thread.delay 0.01;
      loop (i + 1)
    end
  in loop 0

let () =
  let t = Thread.create (fun () -> long_task 10) () in
  Thread.delay 0.035;
  Printf.printf "Sending cancel signal\n";
  cancel ();
  Thread.join t
