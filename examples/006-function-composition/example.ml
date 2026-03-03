(* Function Composition: building pipelines from simple functions *)

(* ── Composition operator (not built-in, but commonly defined) *)

let compose f g x = f (g x)   (* (f ∘ g)(x) = f(g(x)) *)
let ( >> ) f g x = g (f x)    (* pipe/forward composition *)

(* ── Basic examples ──────────────────────────────────────── *)

let double x = x * 2
let inc x = x + 1
let square x = x * x

let double_then_inc = compose inc double   (* inc(double(x)) *)
let inc_then_double = inc >> double        (* double(inc(x)) *)

(* ── Pipeline with |> (OCaml's most-used composition) ──── *)

let process data =
  data
  |> List.map (fun x -> x * 2)
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.fold_left ( + ) 0

(* ── Compose a list of functions ─────────────────────────── *)

let compose_all funcs x =
  List.fold_right (fun f acc -> f acc) funcs x

let pipe_all funcs x =
  List.fold_left (fun acc f -> f acc) x funcs

(* ── Pipeline helper ─────────────────────────────────────── *)

let pipeline value steps =
  List.fold_left (fun acc f -> f acc) value steps

(* ── String processing ───────────────────────────────────── *)

let process_string s =
  s
  |> String.trim
  |> String.lowercase_ascii

(* ── Tests ────────────────────────────────────────────────── *)
let () =
  assert (double_then_inc 5 = 11);
  assert (inc_then_double 5 = 12);
  assert (process [1;2;3;4;5] = 30);
  assert (process [] = 0);
  assert (pipeline 5 [inc; double; (fun x -> x - 3)] = 9);
  assert (compose_all [inc; double; (fun x -> x - 3)] 10 = 15);
  assert (pipe_all [inc; double; (fun x -> x - 3)] 10 = 19);
  assert (process_string "  Hello  " = "hello");
  print_endline "✓ All composition tests passed"
