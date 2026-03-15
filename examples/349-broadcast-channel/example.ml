(* OCaml: broadcast via multiple channels *)

let broadcast subscribers message =
  List.iter (fun ch ->
    Event.sync (Event.send ch message)
  ) subscribers

let () =
  let ch1 = Event.new_channel () in
  let ch2 = Event.new_channel () in
  let ch3 = Event.new_channel () in

  let listener label ch =
    Thread.create (fun () ->
      let msg = Event.sync (Event.receive ch) in
      Printf.printf "%s received: %s\n" label msg
    ) ()
  in

  let t1 = listener "A" ch1 in
  let t2 = listener "B" ch2 in
  let t3 = listener "C" ch3 in

  broadcast [ch1; ch2; ch3] "hello everyone";

  Thread.join t1;
  Thread.join t2;
  Thread.join t3
