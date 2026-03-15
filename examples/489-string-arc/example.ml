(* 489: Shared strings — Arc<str> in Rust / shared references in OCaml *)
(* In OCaml, ALL heap values are reference-counted (managed by the GC) and
   all values are shared by default — there is no distinction between
   Arc (shared) and Box (unique) at the value level.
   OCaml strings are immutable, so sharing is always safe.
   Threads share values via Domain or Thread + Mutex with no extra work. *)

(* Physical equality (==) tests whether two bindings point to the same object,
   analogous to Arc::ptr_eq in Rust. *)
let test_shared () =
  (* s is an immutable string on the heap *)
  let s = "hello" in
  (* s2 is a second binding to the SAME object — no copy *)
  let s2 = s in
  (* Physical equality: both point to the same heap string literal *)
  assert (s == s2);
  Printf.printf "shared reference (ptr_eq): ok\n";
  (* Structural equality also holds *)
  assert (s = s2);
  (* We can "use" both safely — OCaml GC keeps the string alive *)
  assert (String.length s = 5);
  assert (String.length s2 = 5);
  Printf.printf "length via each binding: %d, %d\n"
    (String.length s) (String.length s2)

(* Sending strings across domains (OCaml 5 parallel threads).
   Because OCaml strings are immutable and GC-managed, sharing across
   domains requires no Arc wrapper — just pass the value. *)
let test_cross_domain () =
  let s = "hello" in
  (* Domain.spawn creates a parallel unit of execution *)
  let d = Domain.spawn (fun () ->
    assert (s = "hello");
    String.length s
  ) in
  let len = Domain.join d in
  assert (len = 5);
  Printf.printf "cross-domain string access: len=%d\n" len

(* Demonstrate that String.sub creates a new string (like Arc::from on a slice) *)
let test_sub_creates_copy () =
  let s = "hello world" in
  let sub = String.sub s 0 5 in
  assert (sub = "hello");
  (* sub is a new allocation — not the same object as s *)
  assert (not (sub == s));
  Printf.printf "sub creates new string: ok\n"

let () =
  test_shared ();
  test_cross_domain ();
  test_sub_creates_copy ();
  print_endline "All assertions passed."
