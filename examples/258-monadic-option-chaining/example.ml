(* OCaml option monad: bind and fmap operators *)

(* Monadic bind: >>= propagates None short-circuit *)
let ( >>= ) opt f = match opt with
  | None -> None
  | Some x -> f x

(* Functor map: >>| transforms the value if present *)
let ( >>| ) opt f = match opt with
  | None -> None
  | Some x -> Some (f x)

let safe_div x y = if y = 0 then None else Some (x / y)
let safe_head = function [] -> None | h :: _ -> Some h

(* Idiomatic OCaml: operator chaining *)
let compute lst =
  safe_head lst >>= fun x ->
  safe_div 100 x >>| fun r ->
  r * 2

(* Explicit using Option module — mirrors Rust's and_then + map *)
let compute_explicit lst =
  Option.bind (safe_head lst) (fun x ->
    Option.map (fun r -> r * 2) (safe_div 100 x))

let () =
  let show = function None -> "None" | Some x -> string_of_int x in
  assert (compute [5; 3; 1] = Some 40);
  assert (compute [0; 1] = None);
  assert (compute [] = None);
  assert (compute_explicit [5; 3; 1] = Some 40);
  Printf.printf "%s\n" (show (compute [5; 3; 1]));
  Printf.printf "%s\n" (show (compute [0; 1]));
  Printf.printf "%s\n" (show (compute []));
  print_endline "ok"
