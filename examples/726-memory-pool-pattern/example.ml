(* OCaml: Memory pool / arena pattern
   OCaml's GC acts as an implicit arena for the minor heap.
   We implement an explicit arena for demonstration. *)

(* --- Simple typed pool: pre-allocate N slots --- *)

type 'a pool = {
  slots   : 'a array;
  mutable free: int list;  (* indices of free slots *)
}

exception Pool_exhausted

let make_pool n default =
  { slots = Array.make n default;
    free  = List.init n (fun i -> i) }

let pool_alloc pool x =
  match pool.free with
  | []     -> raise Pool_exhausted
  | i :: t ->
    pool.free <- t;
    pool.slots.(i) <- x;
    i  (* return handle (index) *)

let pool_get pool i = pool.slots.(i)

let pool_free pool i =
  (* Return slot to free list — O(1) *)
  pool.free <- i :: pool.free

let pool_used pool =
  let total = Array.length pool.slots in
  total - List.length pool.free

(* --- Bump allocator (arena) --- *)

type arena = {
  buf  : bytes;
  mutable pos  : int;
  cap  : int;
}

let make_arena cap = { buf = Bytes.create cap; pos = 0; cap }

let arena_alloc arena size =
  if arena.pos + size > arena.cap then failwith "Arena OOM"
  else begin
    let start = arena.pos in
    arena.pos <- arena.pos + size;
    (arena.buf, start, size)   (* reference into arena, not a copy *)
  end

let arena_reset arena = arena.pos <- 0

let arena_used arena = arena.pos

(* --- Parse tree allocated in arena --- *)

type ast_node =
  | Num of int
  | Add of int * int  (* indices into arena-resident nodes *)

let parse_expression arena_buf =
  (* Simulate building a small AST by writing into the arena *)
  let (buf, off, _) = arena_alloc arena_buf 8 in
  Bytes.set_int32_be buf off (Int32.of_int 42);   (* store number *)
  (buf, off)

let () =
  (* Pool demo *)
  let pool = make_pool 4 0 in
  let h1 = pool_alloc pool 100 in
  let h2 = pool_alloc pool 200 in
  Printf.printf "pool[%d]=%d pool[%d]=%d used=%d\n"
    h1 (pool_get pool h1) h2 (pool_get pool h2) (pool_used pool);
  pool_free pool h1;
  Printf.printf "After free h1: used=%d\n" (pool_used pool);
  let h3 = pool_alloc pool 999 in
  Printf.printf "Reused slot %d = %d\n" h3 (pool_get pool h3);

  (* Arena demo *)
  let arena = make_arena 256 in
  let _ = parse_expression arena in
  Printf.printf "Arena used: %d bytes\n" (arena_used arena);
  arena_reset arena;
  Printf.printf "After reset: %d bytes\n" (arena_used arena);

  (* Allocate many small things *)
  for _ = 1 to 10 do
    let _ = arena_alloc arena 8 in ()
  done;
  Printf.printf "After 10×8B allocs: %d bytes used\n" (arena_used arena)
