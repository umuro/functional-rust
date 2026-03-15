(* 728: Inline hints — compiler optimization hints in OCaml *)
(* Rust uses #[inline], #[inline(always)], #[inline(never)], #[cold] to guide LLVM.
   OCaml exposes equivalent mechanisms:

   - [@inline] / [@inline always]:  suggest/force inlining (flambda backend)
   - [@cold]:  not available as a standard attribute; structure cold paths by
     putting them in separate functions (the compiler infers rarely-called paths).
   - [@unrolled]:  available in some contexts for loop unrolling.
   - [@@noalloc] / [@@unboxed]:  C stub attributes, not user-level Rust #[inline].

   The key insight: OCaml's flambda2 optimizer (OCaml 5) performs aggressive
   inlining automatically. The [@inline] attribute is a hint for cross-module
   calls where the optimizer cannot see the body without flambda.

   We demonstrate the same functionality as the Rust example while noting
   where each hint maps. *)

(* [@inline]: suggest inlining — effective across module boundaries with flambda *)
let[@inline] add a b = a + b

(* [@inline always]: force inlining for tiny hot functions *)
let[@inline always] fast_abs x =
  if x < 0 then -x else x

(* [@inline never]: prevent inlining — useful for profiling / large functions.
   OCaml does not have a standard [@inline never] attribute; the closest is
   using a C stub or relying on the optimizer's heuristics. In practice,
   large functions are never inlined automatically. *)
let heavy_computation data =
  Array.fold_left (fun acc x -> acc + x * x) 0 data

(* Cold path: put rarely-executed code in a separate function.
   The compiler sees low call frequency and deprioritizes it.
   In OCaml we document the invariant with a comment. *)
let fail_parse s =
  (* cold path: parsing rarely fails in well-formed input *)
  Printf.eprintf "Failed to parse %S as int, defaulting to 0\n" s;
  0

let parse_int s =
  match int_of_string_opt s with
  | Some v -> v
  | None   -> fail_parse s   (* branch taken rarely → cold in practice *)

(* classify values — hot path first for readability *)
let classify x =
  if x > 0 then "positive"       (* hot: most common *)
  else if x < 0 then "negative"  (* warm *)
  else "zero"                     (* cold: rare *)

(* sum with dispatch — OCaml has no runtime CPU feature detection built-in;
   use the standard float fold as the "portable" implementation *)
let sum_dispatch data =
  Array.fold_left (fun s x -> s + x) 0 data

(* bench_sum: prevent optimizer from eliminating the computation.
   In OCaml, use [Sys.opaque_identity] — the OCaml equivalent of std::hint::black_box *)
let bench_sum data =
  let data = Sys.opaque_identity data in
  Array.fold_left ( + ) 0 data

let () =
  (* inline add *)
  assert (add 3 4 = 7);
  assert (add (-1) 1 = 0);
  print_endline "add: ok";

  (* inline always fast_abs *)
  assert (fast_abs (-42) = 42);
  assert (fast_abs 0 = 0);
  assert (fast_abs 7 = 7);
  print_endline "fast_abs: ok";

  (* heavy_computation: sum of squares *)
  let data = [| 1; 2; 3; 4 |] in
  assert (heavy_computation data = 1 + 4 + 9 + 16);
  print_endline "heavy_computation: ok";

  (* parse_int *)
  assert (parse_int "100" = 100);
  assert (parse_int "0" = 0);
  assert (parse_int "not_a_number" = 0);
  print_endline "parse_int: ok";

  (* classify *)
  assert (classify 5  = "positive");
  assert (classify (-5) = "negative");
  assert (classify 0  = "zero");
  print_endline "classify: ok";

  (* sum_dispatch *)
  let v = Array.init 10 (fun i -> i + 1) in
  assert (sum_dispatch v = 55);
  print_endline "sum_dispatch: ok";

  (* bench_sum *)
  assert (bench_sum [|1;2;3|] = 6);
  print_endline "bench_sum: ok";

  print_endline "All assertions passed."
