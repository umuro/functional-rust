(* Record — Deriving Comparison *)
(* Compare records field by field *)

type date = { year: int; month: int; day: int }

let compare_date a b =
  match compare a.year b.year with
  | 0 -> (match compare a.month b.month with
          | 0 -> compare a.day b.day
          | n -> n)
  | n -> n

let dates = [
  { year=2024; month=3; day=15 };
  { year=2024; month=1; day=20 };
  { year=2023; month=12; day=1 };
  { year=2024; month=3; day=10 };
]

let sorted = List.sort compare_date dates
let () = List.iter (fun d ->
  Printf.printf "%04d-%02d-%02d\n" d.year d.month d.day
) sorted
