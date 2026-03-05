(* OCaml: racing with threads and a channel *)

let race tasks =
  let ch = Event.new_channel () in
  List.iter (fun f ->
    ignore (Thread.create (fun () -> Event.sync (Event.send ch (f ()))) ())
  ) tasks;
  Event.sync (Event.receive ch)

let () =
  let winner = race [
    (fun () -> Thread.delay 0.05; "slow");
    (fun () -> Thread.delay 0.01; "fast");
    (fun () -> Thread.delay 0.03; "medium");
  ] in
  Printf.printf "Winner: %s\n" winner
