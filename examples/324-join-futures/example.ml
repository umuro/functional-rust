(* OCaml: join with threads *)

let parallel tasks =
  let threads = List.map (fun f -> Thread.create f ()) tasks in
  List.iter Thread.join threads

let () =
  parallel [
    (fun () -> Thread.delay 0.05; Printf.printf "A\n");
    (fun () -> Thread.delay 0.03; Printf.printf "B\n");
    (fun () -> Thread.delay 0.01; Printf.printf "C\n");
  ]
