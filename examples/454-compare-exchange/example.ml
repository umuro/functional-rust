(* 454. CAS loops – OCaml 5 *)
let counter = Atomic.make 0

let cas_increment () =
  let rec loop () =
    let cur = Atomic.get counter in
    if not (Atomic.compare_and_set counter cur (cur+1)) then loop ()
  in loop ()

let global_max = Atomic.make min_int

let update_max v =
  let rec loop () =
    let cur = Atomic.get global_max in
    if v > cur && not (Atomic.compare_and_set global_max cur v) then loop ()
  in loop ()

let () =
  let ws = Array.init 4 (fun _ ->
    Domain.spawn (fun () -> for _ = 1 to 100 do cas_increment () done)
  ) in
  Array.iter Domain.join ws;
  Printf.printf "counter=%d\n" (Atomic.get counter);
  List.iter update_max [3;7;1;9;4;6];
  Printf.printf "max=%d\n" (Atomic.get global_max)
