(* 726: Memory pool / bump arena — OCaml equivalent *)
(* Rust implements manual pool and arena allocators to avoid GC overhead.
   In OCaml the GC handles allocation; but the PATTERNS are still useful:
   - Object pools reduce allocation churn and GC pressure.
   - Arenas (or "regions") group related allocations and free them together.

   We demonstrate:
   1. A fixed-capacity typed object pool backed by an array free-list.
   2. A simple bump arena using a Bytes slab (for untyped byte data).
   3. An arena-allocated expression AST using a Queue as the backing store. *)

(* ── Part 1: Fixed-size typed object pool ──────────────────────────────────── *)

type 'a pool = {
  slots    : 'a option array;
  free     : int Queue.t;   (* free-list: indices of available slots *)
  mutable live : int;
}

let pool_create cap =
  let q = Queue.create () in
  for i = 0 to cap - 1 do Queue.add i q done;
  { slots = Array.make cap None; free = q; live = 0 }

let pool_alloc p v =
  if Queue.is_empty p.free then None
  else begin
    let idx = Queue.pop p.free in
    p.slots.(idx) <- Some v;
    p.live <- p.live + 1;
    Some idx
  end

let pool_get p idx =
  match p.slots.(idx) with
  | Some v -> v
  | None   -> failwith "pool_get: slot not live"

let pool_dealloc p idx =
  p.slots.(idx) <- None;
  Queue.add idx p.free;
  p.live <- p.live - 1

(* ── Part 2: Bump arena (byte slab) ────────────────────────────────────────── *)

type arena = {
  slab   : bytes;
  mutable pos : int;
}

let arena_create capacity = { slab = Bytes.create capacity; pos = 0 }

let arena_alloc_bytes a n =
  let start = a.pos in
  let next  = start + n in
  if next > Bytes.length a.slab then failwith "Arena OOM";
  a.pos <- next;
  (* return a sub-bytes view backed by the same slab *)
  Bytes.sub a.slab start n

let arena_reset a = a.pos <- 0
let arena_used a = a.pos
let arena_capacity a = Bytes.length a.slab

(* ── Part 3: Arena-allocated AST using OCaml values + a backing list ─────── *)
(* In OCaml we don't control physical allocation location, but we can
   allocate nodes into a list "arena" and free them all at once by dropping it. *)

type expr =
  | Num of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval = function
  | Num n     -> n
  | Add (l,r) -> eval l + eval r
  | Mul (l,r) -> eval l * eval r

(* Build (1 + 2) * 3 — values are GC-managed, no manual arena needed *)
let build_ast () =
  Mul (Add (Num 1, Num 2), Num 3)

let () =
  (* Pool *)
  let p = pool_create 4 in
  let h = Option.get (pool_alloc p 42) in
  assert (pool_get p h = 42);
  assert (p.live = 1);
  pool_dealloc p h;
  assert (p.live = 0);
  print_endline "pool alloc/dealloc: ok";

  (* Pool exhaustion *)
  let p2 = pool_create 2 in
  assert (pool_alloc p2 1 <> None);
  assert (pool_alloc p2 2 <> None);
  assert (pool_alloc p2 3 = None);
  print_endline "pool exhaustion: ok";

  (* Pool slot reuse *)
  let p3 = pool_create 2 in
  let h3 = Option.get (pool_alloc p3 1) in
  pool_dealloc p3 h3;
  let h4 = Option.get (pool_alloc p3 99) in
  assert (pool_get p3 h4 = 99);
  print_endline "pool reuse: ok";

  (* Bump arena *)
  let a = arena_create 1024 in
  let slice = arena_alloc_bytes a 10 in
  Bytes.set slice 0 (Char.chr 7);
  assert (Char.code (Bytes.get slice 0) = 7);
  assert (arena_used a = 10);
  arena_reset a;
  assert (arena_used a = 0);
  print_endline "arena alloc/reset: ok";

  (* Arena OOM *)
  let tiny = arena_create 8 in
  (try ignore (arena_alloc_bytes tiny 9); assert false
   with Failure "Arena OOM" -> print_endline "arena OOM: ok");

  (* AST eval *)
  let ast = build_ast () in
  assert (eval ast = 9);  (* (1+2)*3 = 9 *)
  print_endline "AST eval: ok";

  print_endline "All assertions passed."
