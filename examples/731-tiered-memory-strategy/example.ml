(* 731: Tiered memory — Stack → Arena → Heap in OCaml *)
(* Rust implements three allocation tiers: stack value, bump-arena slice, heap box.
   OCaml's GC manages all heap values uniformly, but the TIERS still make sense
   as a design pattern:
   Tier 1: unboxed / immediate values (int, bool, char, small tuples with ints)
   Tier 2: arena-backed byte slices (we simulate with a bump arena over Bytes)
   Tier 3: regular GC heap allocation (any value, automatic) *)

(* ── Tier 2: Bump Arena ─────────────────────────────────────────────────── *)

type arena = {
  slab   : bytes;
  mutable offset : int;
}

let arena_create cap = { slab = Bytes.create cap; offset = 0 }

let arena_alloc a n =
  let start = a.offset in
  let next  = start + n in
  if next > Bytes.length a.slab then None
  else begin
    a.offset <- next;
    (* Return a sub-bytes slice backed by the slab *)
    Some (Bytes.sub a.slab start n)
  end

let arena_reset a = a.offset <- 0
let arena_used a = a.offset
let arena_remaining a = Bytes.length a.slab - a.offset

(* ── Tier 3: Heap fallback ─────────────────────────────────────────────── *)

(* In OCaml, Tier 1 = an immediate int (no heap allocation at all).
   Tier 2 = arena slice (Bytes backed by our slab).
   Tier 3 = GC-managed Bytes.create (standard heap allocation). *)
type allocation =
  | Stack of int           (* Tier 1: trivial unboxed value *)
  | Arena of bytes         (* Tier 2: slab-backed slice *)
  | Heap  of bytes         (* Tier 3: GC heap *)

let allocation_as_bytes = function
  | Stack v ->
    let b = Bytes.create 1 in
    Bytes.set b 0 (Char.chr (v land 0xFF));
    b
  | Arena s -> s
  | Heap  b -> b

let allocation_length = function
  | Stack _ -> 1
  | Arena s -> Bytes.length s
  | Heap  b -> Bytes.length b

let tier_alloc arena size =
  if size = 1 then Stack 0
  else match arena_alloc arena size with
  | Some slice -> Arena slice
  | None       -> Heap (Bytes.create size)

let () =
  (* arena alloc and use *)
  let a = arena_create 128 in
  (match arena_alloc a 10 with
  | None -> assert false
  | Some slice ->
    Bytes.set slice 0 (Char.chr 7);
    assert (Char.code (Bytes.get slice 0) = 7);
    assert (arena_used a = 10);
    print_endline "arena alloc and use: ok"
  );

  (* arena full returns None *)
  let a2 = arena_create 8 in
  assert (arena_alloc a2 8 <> None);
  assert (arena_alloc a2 1 = None);
  print_endline "arena full: ok";

  (* arena reset reclaims space *)
  let a3 = arena_create 16 in
  ignore (arena_alloc a3 16);
  assert (arena_remaining a3 = 0);
  arena_reset a3;
  assert (arena_remaining a3 = 16);
  assert (arena_alloc a3 16 <> None);
  print_endline "arena reset: ok";

  (* tier_alloc: heap fallback when arena too small *)
  let small_arena = arena_create 4 in
  let alloc = tier_alloc small_arena 100 in
  assert (allocation_length alloc = 100);
  (match alloc with Heap _ -> print_endline "tier fallback to heap: ok"
   | _ -> assert false);

  (* tier_alloc: uses arena when space available *)
  let big_arena = arena_create 512 in
  ignore (tier_alloc big_arena 10);
  assert (arena_used big_arena = 10);
  print_endline "tier uses arena: ok";

  print_endline "All assertions passed."
