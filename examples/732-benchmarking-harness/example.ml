(* 732: Benchmarking Harness — OCaml stdlib version *)

let time_ns () =
  let t = Unix.gettimeofday () in
  Int64.of_float (t *. 1e9)

let benchmark ?(warmup=10) ?(iters=1000) label f =
  (* Warmup *)
  for _ = 1 to warmup do ignore (f ()) done;
  (* Measure *)
  let times = Array.init iters (fun _ ->
    let t0 = Unix.gettimeofday () in
    ignore (f ());
    let t1 = Unix.gettimeofday () in
    (t1 -. t0) *. 1e6  (* microseconds *)
  ) in
  (* Stats *)
  let n = float_of_int iters in
  let mean = Array.fold_left (+.) 0.0 times /. n in
  let variance = Array.fold_left (fun acc t ->
    let d = t -. mean in acc +. d *. d) 0.0 times /. n in
  let stddev = Float.sqrt variance in
  Printf.printf "%-30s mean=%.2fµs stddev=%.2fµs\n" label mean stddev

let () =
  (* Example: benchmark list creation *)
  benchmark "List.init 1000" (fun () ->
    List.init 1000 (fun i -> i * i));
  benchmark "String.concat" (fun () ->
    String.concat "," (List.init 100 string_of_int))
