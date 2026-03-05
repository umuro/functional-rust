(* Hash, Eq, Ord in OCaml *)

(* Custom ordered type *)
type priority = Low | Medium | High | Critical

let int_of_priority = function
  | Low -> 0 | Medium -> 1 | High -> 2 | Critical -> 3

let compare_priority a b =
  compare (int_of_priority a) (int_of_priority b)

let string_of_priority = function
  | Low -> "Low" | Medium -> "Medium" | High -> "High" | Critical -> "Critical"

type task = {
  name: string;
  priority: priority;
  id: int;
}

let compare_task a b =
  let pc = compare_priority a.priority b.priority in
  if pc <> 0 then -pc  (* Higher priority first *)
  else compare a.name b.name

module TaskMap = Map.Make(struct
  type t = task
  let compare = compare_task
end)

let () =
  let tasks = [
    {name="Fix bug"; priority=Critical; id=1};
    {name="Write docs"; priority=Low; id=2};
    {name="Review PR"; priority=High; id=3};
    {name="Deploy"; priority=High; id=4};
  ] in
  let sorted = List.sort compare_task tasks in
  Printf.printf "Tasks by priority:\n";
  List.iter (fun t ->
    Printf.printf "  [%s] %s\n" (string_of_priority t.priority) t.name
  ) sorted
