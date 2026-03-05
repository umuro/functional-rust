(* Benchmarking with closures in OCaml *)
let time_it label f =
  let start = Unix.gettimeofday () in
  let result = f () in
  let elapsed = Unix.gettimeofday () -. start in
  Printf.printf "%s: %.6f seconds\n" label elapsed;
  result

(* Without Unix, simulate: *)
let benchmark label iterations f =
  let sum = ref 0 in
  for _ = 1 to iterations do
    sum := !sum + (f ())
  done;
  Printf.printf "%s: ran %d iterations (sum=%d to prevent opt)\n" label iterations !sum

let () =
  benchmark "sum_1000" 1000 (fun () ->
    let s = ref 0 in
    for i = 1 to 1000 do s := !s + i done;
    !s
  );
  benchmark "string_ops" 100 (fun () ->
    let s = String.concat "" (List.init 10 (fun i -> string_of_int i)) in
    String.length s
  )
