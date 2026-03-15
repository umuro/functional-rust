(* 731: Tiered Memory — OCaml arena simulation
   OCaml's GC is already generational (young/old heap), but we can
   simulate arena semantics with a Bigarray slab. *)

let () =
  (* Tier 1: Stack-equivalent — OCaml native int/float are unboxed *)
  let x : int = 42 in
  let y : float = 3.14 in
  Printf.printf "Stack-like: %d %.2f\n" x y;

  (* Tier 2: Pool/arena — reuse a Bytes buffer *)
  let arena = Bytes.make 1024 '\x00' in
  let arena_ptr = ref 0 in

  let arena_alloc size =
    let start = !arena_ptr in
    if start + size > Bytes.length arena then failwith "Arena full";
    arena_ptr := start + size;
    (arena, start, size)
  in

  let arena_reset () = arena_ptr := 0 in

  let (buf, off, _sz) = arena_alloc 16 in
  Bytes.blit_string "hello arena" 0 buf off 11;
  Printf.printf "Arena allocation: '%s'\n"
    (Bytes.sub_string buf off 11);

  arena_reset ();
  Printf.printf "Arena reset, ptr=%d\n" !arena_ptr;

  (* Tier 3: Heap — normal OCaml allocation *)
  let heap_data = Array.make 10000 0 in
  heap_data.(0) <- 99;
  Printf.printf "Heap: heap_data[0]=%d len=%d\n"
    heap_data.(0) (Array.length heap_data)
