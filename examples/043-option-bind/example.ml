(* Option Bind *)
(* OCaml 99 Problems #43 *)

let safe_div a b = if b = 0 then None else Some (a / b)

let chained_div a b c = Option.bind (safe_div a b) (fun x -> safe_div x c)

(* Tests *)
let () =
  assert (safe_div 10 2 = Some 5);
  assert (safe_div 10 0 = None);
  assert (chained_div 100 5 2 = Some 10);
  assert (chained_div 100 0 2 = None);
  assert (chained_div 100 5 0 = None);
  print_endline "✓ OCaml tests passed"
