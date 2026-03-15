(* 453. Memory ordering – OCaml 5 note *)
(* All OCaml 5 atomics are sequentially consistent *)
let data  = Array.make 10 0
let ready = Atomic.make false

let producer () =
  Array.iteri (fun i _ -> data.(i) <- i*i) data;
  Atomic.set ready true  (* implicit Release *)

let consumer () =
  while not (Atomic.get ready) do () done;  (* implicit Acquire *)
  Printf.printf "sum=%d\n" (Array.fold_left (+) 0 data)

let () =
  let p = Domain.spawn producer in
  let c = Domain.spawn consumer in
  Domain.join p; Domain.join c
