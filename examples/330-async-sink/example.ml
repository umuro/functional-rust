(* OCaml: sink-like accumulator with flush *)

type 'a sink = { mutable buf: 'a list; cap: int; flush_fn: 'a list -> unit }

let make_sink cap f = { buf=[]; cap; flush_fn=f }

let send s x =
  s.buf <- x :: s.buf;
  if List.length s.buf >= s.cap then (s.flush_fn (List.rev s.buf); s.buf <- [])

let flush s = if s.buf <> [] then (s.flush_fn (List.rev s.buf); s.buf <- [])

let () =
  let out = ref 0 in
  let s = make_sink 3 (fun b -> out := !out + 1; Printf.printf "Flush %d items\n" (List.length b)) in
  List.iter (send s) [1;2;3;4;5;6;7;8];
  flush s;
  Printf.printf "Batches: %d\n" !out
