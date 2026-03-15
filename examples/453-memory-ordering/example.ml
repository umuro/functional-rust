(* 453: Memory Ordering
   OCaml 5's Atomic module uses sequentially-consistent (SC) semantics
   by default — there is no explicit Relaxed/Acquire/Release knob
   exposed in the stdlib (the runtime's GC enforces SC for atomics).
   We demonstrate the release-acquire handshake pattern, which is
   the primary use-case for memory ordering in Rust. *)

let () =
  let data  = Atomic.make 0 in
  let flag  = Atomic.make false in

  (* Writer domain: store data, then set flag *)
  let writer = Domain.spawn (fun () ->
    Atomic.set data 42;   (* "relaxed" store — safe because SC *)
    Atomic.set flag true  (* "release" — everything before this is visible
                             to any thread that observes flag = true *)
  ) in
  Domain.join writer;

  (* Reader: flag.load() acts as an "acquire" fence *)
  assert (Atomic.get flag = true);
  assert (Atomic.get data = 42);
  Printf.printf "release-acquire handshake: flag=%b data=%d\n%!"
    (Atomic.get flag) (Atomic.get data);

  (* Relaxed counter — no ordering needed for a simple tally *)
  let counter = Atomic.make 0 in
  let domains = List.init 4 (fun _ ->
    Domain.spawn (fun () ->
      for _ = 1 to 100 do
        (* fetch_and_add is SC in OCaml; analogous to fetch_add(Relaxed) *)
        ignore (Atomic.fetch_and_add counter 1)
      done))
  in
  List.iter Domain.join domains;
  assert (Atomic.get counter = 400);
  Printf.printf "concurrent counter: %d\n%!" (Atomic.get counter)
