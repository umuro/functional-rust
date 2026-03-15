(* 849: Higher-Order Functions — the foundation of functional-style programming *)

(* ── Passing functions as arguments ──────────────────────────────────────── *)

(* Apply a function to a value — OCaml: let apply f x = f x *)
let apply f x = f x

(* Apply a function to a value twice — OCaml: let twice f x = f (f x) *)
let twice f x = f (f x)

(* ── Returning functions (closures) ──────────────────────────────────────── *)

(* Return a function that adds n to its argument *)
let adder n = fun x -> x + n

(* Return a function that multiplies its argument by n *)
let multiplier n = fun x -> x * n

(* ── Function composition ────────────────────────────────────────────────── *)

(* compose f g x = f (g x) — OCaml: let compose f g x = f (g x) *)
let compose f g x = f (g x)

(* Pipe a value through a list of transformations — OCaml's |> generalised *)
let pipe value fns =
  List.fold_left (fun acc f -> f acc) value fns

(* ── Iterator combinators (map / filter / fold) ──────────────────────────── *)

let double_all xs = List.map (fun x -> x * 2) xs

let keep_evens xs = List.filter (fun x -> x mod 2 = 0) xs

let sum xs = List.fold_left ( + ) 0 xs

(* Generic higher-order: map, then filter, then fold — all in one expression *)
let map_filter_fold xs map_fn filter_fn init fold_fn =
  let mapped   = List.map map_fn xs in
  let filtered = List.filter filter_fn mapped in
  List.fold_left fold_fn init filtered

(* ── Demo ────────────────────────────────────────────────────────────────── *)

let () =
  (* apply *)
  assert (apply (fun x -> x * 3) 7 = 21);
  assert (apply String.length "hello" = 5);

  (* twice *)
  assert (twice (fun x -> x + 1) 5 = 7);
  assert (twice (fun x -> x * 2) 3 = 12);

  (* compose *)
  let double_after_inc = compose (fun x -> x * 2) (fun x -> x + 1) in
  assert (double_after_inc 5 = 12);  (* (5+1)*2 *)
  assert (double_after_inc 0 = 2);   (* (0+1)*2 *)

  (* adder / multiplier *)
  let add5   = adder 5 in
  let times3 = multiplier 3 in
  assert (add5 10 = 15);
  assert (times3 4 = 12);

  (* pipe — OCaml's |> operator is built-in, but here we use explicit list *)
  let inc = adder 1 in
  let dbl = multiplier 2 in
  assert (pipe 5 [inc; dbl] = 12);  (* (5+1)*2 *)
  assert (pipe 0 [inc; dbl] = 2);
  assert (pipe 5 []          = 5);  (* identity *)

  (* iterator combinators *)
  let xs = [1; 2; 3; 4; 5] in
  assert (double_all xs = [2; 4; 6; 8; 10]);
  assert (keep_evens xs = [2; 4]);
  assert (sum xs = 15);

  (* map_filter_fold: square [1..5], keep > 10, sum → 16+25 = 41 *)
  let result = map_filter_fold
    [1; 2; 3; 4; 5]
    (fun x -> x * x)
    (fun x -> x > 10)
    0
    ( + )
  in
  assert (result = 41);

  (* Idiomatic OCaml pipeline with |> *)
  let result2 =
    [1; 2; 3; 4; 5]
    |> List.map (fun x -> x * x)
    |> List.filter (fun x -> x > 10)
    |> List.fold_left ( + ) 0
  in
  assert (result2 = 41);

  print_endline "849-higher-order-functions: all tests passed"
