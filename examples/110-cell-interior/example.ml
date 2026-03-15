(* Example 110: Cell<T> Interior Mutability — OCaml ref → Rust Cell *)

(* Approach 1: Simple mutable counter with ref *)
let approach1 () =
  let counter = ref 0 in
  counter := !counter + 1;
  counter := !counter + 1;
  assert (!counter = 2);
  Printf.printf "Counter: %d\n" !counter

(* Approach 2: Mutable field in an immutable struct *)
type config = { name : string; mutable call_count : int }

let use_config c =
  c.call_count <- c.call_count + 1;
  Printf.printf "Config '%s' used %d times\n" c.name c.call_count

let approach2 () =
  let c = { name = "default"; call_count = 0 } in
  use_config c;
  use_config c;
  assert (c.call_count = 2)

(* Approach 3: Caching/memoization with ref *)
let lazy_value init =
  let cache = ref None in
  fun () ->
    match !cache with
    | Some v -> v
    | None ->
      let v = init () in
      cache := Some v;
      v

let approach3 () =
  let calls = ref 0 in
  let expensive = lazy_value (fun () -> incr calls; 42) in
  let v1 = expensive () in
  let v2 = expensive () in
  assert (v1 = 42);
  assert (v2 = 42);
  assert (!calls = 1);
  Printf.printf "Computed once: %d (calls: %d)\n" v1 !calls

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
