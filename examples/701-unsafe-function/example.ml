(* 701: Unsafe functions — safe wrappers in OCaml *)
(* Rust's unsafe fn marks functions whose callers must uphold invariants
   that the type system cannot check.

   OCaml equivalents:
   - "Unsafe" byte-copying uses Bytes.blit — the stdlib function is safe because
     it checks bounds and the GC manages lifetimes.
   - Array.unsafe_get / Array.unsafe_blit skip bounds checks; callers are
     responsible for staying in bounds (analogous to unsafe fn in Rust).
   - The safe-wrapper pattern: validate inputs, then call the unchecked function.

   Key difference: OCaml's "unsafe" operations (Array.unsafe_get etc.) cannot
   cause memory corruption — the GC still owns the memory. They only risk
   reading stale/wrong data if you violate the size invariant. *)

(* Safe copy: validate lengths, then delegate to Bytes.blit *)
let safe_copy src dst =
  let src_len = Bytes.length src in
  let dst_len = Bytes.length dst in
  if src_len <> dst_len then
    Error (Printf.sprintf "length mismatch: src=%d dst=%d" src_len dst_len)
  else begin
    Bytes.blit src 0 dst 0 src_len;  (* equivalent of raw_copy in Rust *)
    Ok ()
  end

(* Safe indexed get: bounds-check before calling the unchecked variant *)
let safe_get arr idx =
  if idx < Array.length arr then
    Some (Array.unsafe_get arr idx)  (* unchecked after validation *)
  else
    None

(* Demonstrate the pattern: document invariants, then use unchecked inner call *)

(* Inner function — only safe when 0 <= idx < Array.length arr.
   Called only through safe_get above. *)
let _unsafe_get arr idx = Array.unsafe_get arr idx

let () =
  (* safe_copy *)
  let src = Bytes.of_string "abcde" in
  let dst = Bytes.create 5 in
  assert (safe_copy src dst = Ok ());
  assert (Bytes.to_string dst = "abcde");
  print_endline "safe_copy: ok";

  (* mismatch *)
  assert (Result.is_error (safe_copy (Bytes.of_string "abc") (Bytes.create 5)));
  print_endline "safe_copy mismatch: ok";

  (* safe_get *)
  let v = [| 1; 2; 3 |] in
  assert (safe_get v 0 = Some 1);
  assert (safe_get v 2 = Some 3);
  assert (safe_get v 3 = None);
  print_endline "safe_get: ok";

  print_endline "All assertions passed."
