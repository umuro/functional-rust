(* String matching in OCaml *)
let cmd s = match s with
  | "quit" | "exit" | "q" -> "quit"
  | "help" | "?" | "h"    -> "help"
  | s when String.length s > 0 && s.[0] = '/' -> "command"
  | ""                     -> "empty"
  | _                      -> "unknown"

let day_type = function
  | "Monday"|"Tuesday"|"Wednesday"|"Thursday"|"Friday" -> "weekday"
  | "Saturday"|"Sunday"                                -> "weekend"
  | _                                                  -> "unknown"

let () =
  List.iter (fun s -> Printf.printf "'%s'->'%s'\n" s (cmd s))
    ["quit";"help";"/run";"";"foo"];
  List.iter (fun d -> Printf.printf "%s:%s\n" d (day_type d))
    ["Monday";"Saturday";"Holiday"]
