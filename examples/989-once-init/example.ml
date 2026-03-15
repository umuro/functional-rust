(* 989: One-Time Initialization *)
(* OCaml: Lazy.t — computed at most once, memoized *)

(* --- Approach 1: Lazy.t for deferred initialization --- *)

let expensive_config = lazy (
  (* Simulated expensive computation *)
  Printf.printf "(computing config...)\n";
  { contents = 42 }
)

let get_config () = Lazy.force expensive_config

let () =
  (* First access: computes *)
  let c1 = get_config () in
  (* Second access: returns cached value *)
  let c2 = get_config () in
  assert (c1 == c2);  (* physical equality: same object *)
  assert (!c1 = 42);
  Printf.printf "Approach 1 (Lazy.t): %d\n" !c1

(* --- Approach 2: Thread-safe once-init with Mutex + option --- *)

type 'a once = {
  mutable value: 'a option;
  m: Mutex.t;
}

let make_once () = { value = None; m = Mutex.create () }

let once_get once f =
  Mutex.lock once.m;
  let v = match once.value with
    | Some v -> v
    | None ->
      let v = f () in
      once.value <- Some v;
      v
  in
  Mutex.unlock once.m;
  v

let db_connection = make_once ()

let get_db () = once_get db_connection (fun () ->
  Printf.printf "(opening DB connection...)\n";
  "conn://localhost:5432"
)

let () =
  let c1 = get_db () in
  let c2 = get_db () in
  assert (c1 = c2);
  assert (c1 = "conn://localhost:5432");
  Printf.printf "Approach 2 (thread-safe once): %s\n" c1

(* --- Approach 3: Lazy initialization in module --- *)

let _initialized = lazy (
  (* Module-level initialization — runs once *)
  Printf.printf "(module init)\n";
  true
)

let ensure_init () = Lazy.force _initialized

let () =
  let r1 = ensure_init () in
  let r2 = ensure_init () in
  assert (r1 = r2);
  Printf.printf "Approach 3 (module lazy): %b\n" r1

let () = Printf.printf "✓ All tests passed\n"
