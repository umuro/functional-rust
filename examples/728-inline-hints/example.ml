(* OCaml: Inlining and cold-path hints
   OCaml has limited inlining control. `[@unrolled]` is for loops.
   flambda (the optimising middle-end) inlines aggressively, but
   you can't mark specific functions as cold. *)

(* --- Inlining: OCaml inlines small functions automatically with flambda --- *)

(* Small functions get inlined by the OCaml native compiler *)
let[@inline] add x y = x + y
let[@inline] square x = x * x
let[@inline always] fast_abs x = if x < 0 then -x else x

(* Without [@inline]: compiler decides based on size and heuristics *)
let slow_abs x = if x < 0 then -x else x

(* --- Cold path: not directly available in OCaml --- *)
(* We simulate the concept: rare paths in separate functions *)

let[@cold] handle_error msg =
  (* This function is unlikely to be called; OCaml doesn't have @[cold]
     but we document the intent. Rust's #[cold] biases branch prediction. *)
  Printf.eprintf "Error: %s\n" msg

(* Hot path using a result type — error branch is rare *)
let divide x y =
  if y = 0 then begin
    handle_error "division by zero";
    None
  end else
    Some (x / y)

(* --- Target features: not available in standard OCaml --- *)
(* Would need C FFI with `__attribute__((target("avx2")))` *)

(* Closest: use the BLAS/LAPACK bindings which use SIMD internally *)
(* e.g., owl-base calls cblas_sdot which uses SIMD at runtime *)

(* --- Benchmark: inlined vs not inlined --- *)
let time_it label f =
  let t0 = Sys.time () in
  let r = f () in
  Printf.printf "%s: %.6fs result=%d\n" label (Sys.time () -. t0) r

let () =
  let n = 50_000_000 in

  time_it "[@inline] add (cumulative)" (fun () ->
    let acc = ref 0 in
    for i = 1 to n do acc := add !acc i done;
    !acc);

  time_it "[@inline always] fast_abs" (fun () ->
    let acc = ref 0 in
    for i = -n to n do acc := !acc + fast_abs i done;
    !acc);

  time_it "slow_abs (no inline hint)" (fun () ->
    let acc = ref 0 in
    for i = -n to n do acc := !acc + slow_abs i done;
    !acc);

  (* Demonstrate cold path *)
  let results = [divide 10 2; divide 5 0; divide 100 4] in
  List.iter (function
    | None -> ()
    | Some v -> Printf.printf "Result: %d\n" v
  ) results;

  Printf.printf "Note: flambda inlines aggressively; for SIMD use C FFI.\n"
