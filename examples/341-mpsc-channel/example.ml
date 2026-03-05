(* OCaml: Channel communication *)

let () =
  let ch = Event.new_channel () in
  for i = 0 to 3 do
    ignore (Thread.create (fun () ->
      Event.sync (Event.send ch (Printf.sprintf "message from %d" i))
    ) ())
  done;
  for _ = 0 to 3 do
    Printf.printf "%s\n" (Event.sync (Event.receive ch))
  done
