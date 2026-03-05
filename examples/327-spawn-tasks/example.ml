(* OCaml: spawning workers *)

let spawn_worker id delay =
  Thread.create (fun () ->
    Thread.delay delay;
    Printf.sprintf "worker-%d done" id
  ) ()

let () =
  let handles = List.init 5 (fun i -> (i, spawn_worker i (float_of_int (5-i) *. 0.01))) in
  List.iter (fun (id, t) -> Thread.join t; Printf.printf "Worker %d finished\n" id) handles
