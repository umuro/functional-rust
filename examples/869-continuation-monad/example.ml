(* 869: Continuation Monad — Delimited Continuations in OCaml

   The continuation monad wraps computations that pass their result to a callback.
   type ('a, 'r) cont = ('a -> 'r) -> 'r

   In OCaml, closures are first-class and uniform, so the encoding is clean
   without the Box<dyn Fn> gymnastics needed in Rust. *)

(* ── Core type and operations ─────────────────────────────────────────────── *)

(* A continuation computation: given a callback k, produce a result *)
type ('a, 'r) cont = { run_cont : ('a -> 'r) -> 'r }

(* Wrap a pure value: \k -> k a *)
let return_cont a = { run_cont = fun k -> k a }

(* Bind: sequence continuations — m >>= f *)
let bind_cont m f = { run_cont = fun k -> m.run_cont (fun a -> (f a).run_cont k) }

(* Run: supply the final continuation *)
let run_cont c k = c.run_cont k

(* Idiomatic let-binding syntax for continuation chains *)
let ( let* ) = bind_cont

(* ── CPS (Continuation-Passing Style) — direct functions ──────────────────── *)

(* CPS add: calls k with the sum *)
let cps_add a b k = k (a + b)

(* CPS factorial — tail-recursive via continuations *)
let rec fact_cps n k =
  if n <= 1 then k 1
  else fact_cps (n - 1) (fun r -> k (n * r))

(* CPS Fibonacci *)
let rec fib_cps n k =
  if n <= 1 then k n
  else
    fib_cps (n - 1) (fun a ->
    fib_cps (n - 2) (fun b ->
    k (a + b)))

(* ── Using the monad ──────────────────────────────────────────────────────── *)

(* Monadic computation: add two numbers and double the result *)
let add_and_double a b =
  let* x = return_cont a in
  let* y = return_cont b in
  return_cont ((x + y) * 2)

(* ── callCC — call with current continuation ──────────────────────────────── *)

(* callCC: capture the current continuation for early exit *)
let call_cc f = { run_cont = fun k -> (f (fun a -> { run_cont = fun _ -> k a })).run_cont k }

(* Early exit example: sum a list, but bail out if a zero is found *)
let sum_no_zeros lst =
  run_cont
    (call_cc (fun exit ->
      List.fold_left (fun acc_c x ->
        let* acc = acc_c in
        if x = 0 then exit (-1)   (* early exit with sentinel *)
        else return_cont (acc + x)
      ) (return_cont 0) lst))
    (fun x -> x)

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* Direct CPS *)
  let r = cps_add 3 4 (fun x -> x) in
  assert (r = 7);

  let r2 = fact_cps 5 (fun x -> x) in
  assert (r2 = 120);

  let r3 = fib_cps 10 (fun x -> x) in
  assert (r3 = 55);

  (* Monad return *)
  assert (run_cont (return_cont 42) (fun x -> x) = 42);
  assert (run_cont (return_cont 10) (fun x -> x * 2) = 20);

  (* Monadic bind chain *)
  let result =
    run_cont
      (let* x = return_cont 10 in
       let* y = return_cont (x * 2) in
       return_cont (y + 1))
      (fun x -> x)
  in
  assert (result = 21);

  (* add_and_double *)
  assert (run_cont (add_and_double 3 4) (fun x -> x) = 14);

  (* callCC early exit *)
  assert (sum_no_zeros [1; 2; 3; 4; 5] = 15);
  assert (sum_no_zeros [1; 2; 0; 4; 5] = -1);  (* bailed out *)

  print_endline "869-continuation-monad: all tests passed"
