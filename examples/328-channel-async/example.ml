(* OCaml: message passing with channels *)

let () =
  let ch = Event.new_channel () in
  let prod label n = Thread.create (fun () ->
    for i = 1 to n do Event.sync (Event.send ch (Printf.sprintf "%s-%d" label i)) done
  ) () in
  let _t1 = prod "A" 3 in
  let _t2 = prod "B" 3 in
  for _ = 1 to 6 do Printf.printf "Recv: %s\n" (Event.sync (Event.receive ch)) done
