(* 441. Thread basics – OCaml *)
let () =
  let handles = Array.init 4 (fun i ->
    Thread.create (fun () ->
      let r = i * i in
      Printf.printf "Thread %d: %d^2 = %d\n%!" i i r;
      r
    ) ()
  ) in
  (* OCaml Thread.join returns unit, not the result *)
  Array.iter Thread.join handles;
  Printf.printf "All threads done\n"
