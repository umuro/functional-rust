(* OCaml: closest concept to a custom allocator is tracking GC stats.
   OCaml doesn't expose a custom-allocator API in the standard library,
   but we can demonstrate the *intent*: count allocations and bytes. *)

(* GC statistics (built-in) *)
let print_gc_stats label =
  let s = Gc.stat () in
  Printf.printf "[%s] minor_words=%.0f major_words=%.0f live_words=%d\n"
    label s.Gc.minor_words s.Gc.major_words s.Gc.live_words

(* Simulate "tracked allocation" with a ref-counted wrapper *)
let total_allocated = ref 0

let tracked_alloc n =
  total_allocated := !total_allocated + n;
  Bytes.create n   (* OCaml allocation — goes through GC *)

let tracked_free _buf n =
  (* GC manages lifetime; this just tracks intent *)
  total_allocated := !total_allocated - n

let () =
  print_gc_stats "start";
  let bufs = Array.init 1000 (fun _ -> tracked_alloc 64) in
  print_gc_stats "after alloc";
  Printf.printf "Tracked live bytes: %d\n" !total_allocated;
  Array.iter (fun b -> tracked_free b 64) bufs;
  Printf.printf "After free: tracked=%d\n" !total_allocated;
  print_gc_stats "end";

  (* OCaml note: real deallocation happens whenever GC collects;
     a custom allocator in Rust would immediately reclaim on dealloc. *)
  ()

(* --- Conceptual bump allocator (as a pure functional model) --- *)

type bump_state = { buf: bytes; mutable pos: int; capacity: int }

let make_bump cap = { buf = Bytes.create cap; pos = 0; capacity = cap }

let bump_alloc state size =
  if state.pos + size > state.capacity then None
  else begin
    let ptr = state.pos in
    state.pos <- state.pos + size;
    Some ptr   (* return "offset" as a pointer handle *)
  end

let bump_reset state = state.pos <- 0

let () =
  let arena = make_bump 256 in
  (match bump_alloc arena 100 with
  | Some p -> Printf.printf "Bump alloc at offset %d\n" p
  | None   -> print_endline "OOM");
  (match bump_alloc arena 100 with
  | Some p -> Printf.printf "Bump alloc at offset %d\n" p
  | None   -> print_endline "OOM (expected: no space)");
  bump_reset arena;
  (match bump_alloc arena 50 with
  | Some p -> Printf.printf "After reset, bump alloc at offset %d\n" p
  | None   -> print_endline "OOM")
