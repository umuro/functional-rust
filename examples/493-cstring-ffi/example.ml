(* 493: CString / CStr — FFI strings in OCaml *)
(* Rust's CString ensures null-termination and no interior nulls for C FFI.
   OCaml's Foreign Function Interface (FFI) uses `Bytes` and the `ctypes`
   library or the low-level `Bigarray` for C interop.
   The standard `Callback` / `Foreign` / `Bigarray` approach:
   - `Bytes.create (n+1)` gives a null-padded buffer.
   - OCaml strings passed to C via `String.unsafe_to_bytes` are already
     null-terminated when allocated (OCaml guarantees a trailing \0).

   Here we demonstrate the OCaml equivalents of CString operations. *)

(* Create a null-terminated byte buffer from a string.
   Fails (None) if the string contains an interior null byte. *)
let cstring_new s =
  if String.contains s '\000' then None
  else begin
    let n = String.length s in
    let buf = Bytes.create (n + 1) in
    Bytes.blit_string s 0 buf 0 n;
    Bytes.set buf n '\000';  (* null terminator *)
    Some buf
  end

(* Check that the last byte is the null terminator *)
let cstring_has_null_terminator buf =
  let n = Bytes.length buf in
  n > 0 && Bytes.get buf (n - 1) = '\000'

(* Extract the string without the null terminator *)
let cstring_to_string buf =
  let n = Bytes.length buf in
  if n = 0 then ""
  else
    (* find null or use full length *)
    let len = ref (n - 1) in
    (try
      for i = 0 to n - 1 do
        if Bytes.get buf i = '\000' then begin
          len := i;
          raise Exit
        end
      done
    with Exit -> ());
    Bytes.sub_string buf 0 !len

(* as_bytes_with_nul: return the full buffer including null *)
let as_bytes_with_nul buf = Bytes.to_string buf

let () =
  (* new: valid string *)
  assert (cstring_new "hello" <> None);
  print_endline "cstring_new valid: ok";

  (* new: interior null — should fail *)
  assert (cstring_new "hel\000lo" = None);
  print_endline "cstring_new interior null: rejected ok";

  (* roundtrip *)
  (match cstring_new "hi" with
  | None -> assert false
  | Some buf ->
    let s = cstring_to_string buf in
    assert (s = "hi");
    Printf.printf "roundtrip: %s\n" s;

    (* null byte at end *)
    assert (cstring_has_null_terminator buf);
    let raw = as_bytes_with_nul buf in
    assert (raw.[String.length raw - 1] = '\000');
    print_endline "null terminator present: ok"
  );

  print_endline "All assertions passed."
