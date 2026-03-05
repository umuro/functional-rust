(* 451. select! concept – OCaml Event *)
let () =
  let ch1 = Event.new_channel () in
  let ch2 = Event.new_channel () in
  ignore (Thread.create (fun () ->
    Thread.delay 0.02;
    Event.sync (Event.send ch1 "from ch1")) ());
  ignore (Thread.create (fun () ->
    Thread.delay 0.01;
    Event.sync (Event.send ch2 "from ch2")) ());
  for _ = 1 to 2 do
    let msg = Event.sync (Event.choose [
      Event.receive ch1; Event.receive ch2]) in
    Printf.printf "got: %s\n%!" msg
  done
