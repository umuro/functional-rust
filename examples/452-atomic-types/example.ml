(* 452. Atomic types – OCaml 5 *)
let counter = Atomic.make 0
let running  = Atomic.make true

let () =
  let workers = Array.init 4 (fun _ ->
    Domain.spawn (fun () ->
      while Atomic.get running do
        ignore (Atomic.fetch_and_add counter 1)
      done)
  ) in
  Unix.sleepf 0.005;
  Atomic.set running false;
  Array.iter Domain.join workers;
  Printf.printf "counter = %d\n" (Atomic.get counter)
