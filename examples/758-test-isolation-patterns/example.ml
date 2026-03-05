(* 758: Test Isolation — OCaml *)
(* OCaml tests run sequentially by default (no threads), but good habits apply *)

(* BAD: global mutable state shared across tests *)
let global_counter = ref 0

let bad_test_1 () =
  global_counter := 0;  (* Must reset — fragile! *)
  incr global_counter;
  assert (!global_counter = 1)

let bad_test_2 () =
  (* If bad_test_1 didn't reset, this fails! *)
  global_counter := 0;
  global_counter := !global_counter + 5;
  assert (!global_counter = 5)

(* GOOD: per-test state — no sharing *)
let make_counter () = ref 0

let good_test_1 () =
  let c = make_counter () in
  incr c;
  assert (!c = 1)

let good_test_2 () =
  let c = make_counter () in
  c := !c + 5;
  assert (!c = 5)

(* GOOD: read-only shared state via lazy *)
let shared_data = lazy (
  Printf.printf "[init] Building shared test data\n";
  Array.init 100 (fun i -> i * i)
)

let shared_test_1 () =
  let data = Lazy.force shared_data in
  assert (data.(3) = 9)

let shared_test_2 () =
  let data = Lazy.force shared_data in
  assert (data.(10) = 100)

let () =
  bad_test_1 ();
  bad_test_2 ();
  good_test_1 ();
  good_test_2 ();
  shared_test_1 ();
  shared_test_2 ();
  Printf.printf "Isolation tests passed!\n"
