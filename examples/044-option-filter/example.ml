(* Option Filter *)
(* OCaml 99 Problems #44 *)

(* Option.filter is not in the stdlib Option module; derive it from bind,
   the same derivation the OCaml Approach notes give for pre-4.08 code. *)
let option_filter pred opt = Option.bind opt (fun x -> if pred x then Some x else None)

let filter_positive_even opt =
  opt |> option_filter (fun x -> x > 0) |> option_filter (fun x -> x mod 2 = 0)

(* Tests *)
let () =
  assert (filter_positive_even (Some 4) = Some 4);
  assert (filter_positive_even (Some (-4)) = None);
  assert (filter_positive_even (Some 3) = None);
  assert (filter_positive_even None = None);
  print_endline "✓ OCaml tests passed"
