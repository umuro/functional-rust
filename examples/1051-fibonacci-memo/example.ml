(* 1051: Fibonacci with Memoization
   Top-down recursive fib with a Hashtbl cache — same idea as Rust's HashMap memo. *)

(* Approach 1: Recursive with explicit Hashtbl cache *)
let fib_memo =
  let cache = Hashtbl.create 64 in
  let rec fib n =
    if n <= 1 then n
    else match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
      let v = fib (n - 1) + fib (n - 2) in
      Hashtbl.add cache n v;
      v
  in
  fib

(* Approach 2: Functional memoizer using a local cache in a closure *)
let make_fib_memo () =
  let cache = Hashtbl.create 64 in
  let rec fib n =
    if n <= 1 then n
    else match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
      let v = fib (n - 1) + fib (n - 2) in
      Hashtbl.add cache n v; v
  in
  fib

(* Approach 3: Generic memoize combinator using a Hashtbl *)
let memoize f =
  let cache = Hashtbl.create 64 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v; v

(* To memoize recursive functions, use the open-recursion trick *)
let fib_open recurse n =
  if n <= 1 then n
  else recurse (n - 1) + recurse (n - 2)

(* Y-combinator style via rec + memoize *)
let fib_memoized =
  let cache = Hashtbl.create 64 in
  let rec fib n =
    if n <= 1 then n
    else match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
      let v = fib_open fib n in
      Hashtbl.add cache n v; v
  in
  fib

let () =
  let cases = [(0,0);(1,1);(10,55);(20,6765);(30,832040)] in
  List.iter (fun (n, expected) ->
    assert (fib_memo n = expected)
  ) cases;

  let fib = make_fib_memo () in
  List.iter (fun (n, expected) ->
    assert (fib n = expected)
  ) cases;

  List.iter (fun (n, expected) ->
    assert (fib_memoized n = expected)
  ) cases;

  (* Demonstrate generic memoize on a simple function *)
  let call_count = ref 0 in
  let expensive x = incr call_count; x * x in
  let memo_exp = memoize expensive in
  assert (memo_exp 5 = 25);
  assert (memo_exp 5 = 25);  (* cached *)
  assert (!call_count = 1);  (* only computed once *)

  Printf.printf "All fibonacci-memo tests passed.\n"
