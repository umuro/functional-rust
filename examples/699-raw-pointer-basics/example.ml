(* 699: Raw pointer basics — OCaml's safe equivalent *)
(* Rust has *const T and *mut T raw pointers requiring unsafe blocks.
   OCaml is memory-safe by design: there are no raw pointers exposed to user code.
   The GC manages all heap objects; the type system prevents dangling references.

   The OCaml equivalent of "safe_read via raw pointer" is:
   - Array / Bigarray for contiguous typed data
   - Array.get with bounds checking (default, O(1))
   - Array.unsafe_get to skip bounds check (analogous to raw pointer arithmetic,
     but still within the bounds of a typed array — NOT truly unsafe at the OS level)

   We demonstrate both the safe and the unchecked API. *)

(* Safe indexed read — analogous to Rust's safe_read wrapping a raw pointer *)
let safe_read arr idx =
  if idx >= Array.length arr then None
  else Some arr.(idx)

(* Unchecked read — skip OCaml's bounds check for performance in hot loops.
   INVARIANT: caller must guarantee idx < Array.length arr.
   This is NOT unsafe in the OS/memory-corruption sense — the GC still owns
   the array; we just skip the bounds-check branch. *)
let unchecked_read arr idx =
  Array.unsafe_get arr idx

(* Mutable write — OCaml arrays are mutable; no raw pointer needed *)
let write arr idx value =
  arr.(idx) <- value

(* Demonstrate Bigarray for C-interop style typed memory *)
let () =
  (* safe_read *)
  let arr = [| 10l; 20l; 30l |] in  (* int32 array *)
  assert (safe_read arr 0 = Some 10l);
  assert (safe_read arr 2 = Some 30l);
  assert (safe_read arr 3 = None);
  assert (safe_read arr max_int = None);
  print_endline "safe_read: ok";

  (* unchecked_read — caller guarantees idx in bounds *)
  assert (unchecked_read arr 1 = 20l);
  print_endline "unchecked_read: ok";

  (* mutable write — analogous to *mut T write *)
  let mut_arr = [| 7L |] in   (* int64 array *)
  write mut_arr 0 42L;
  assert (mut_arr.(0) = 42L);
  print_endline "mutable write: ok";

  (* byte-level access via Bytes — closest OCaml analog to *const u8 *)
  let data = Bytes.of_string "\x64\xC8\x96" in (* 100, 200, 150 *)
  let second = Char.code (Bytes.get data 1) in
  assert (second = 200);
  Printf.printf "byte[1] = %d\n" second;

  print_endline "All assertions passed."
