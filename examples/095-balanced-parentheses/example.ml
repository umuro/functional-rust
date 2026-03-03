let is_balanced s =
  let matching = function ')' -> '(' | ']' -> '[' | '}' -> '{' | _ -> ' ' in
  let rec check stack i =
    if i = String.length s then stack = []
    else match s.[i] with
    | '(' | '[' | '{' as c -> check (c :: stack) (i + 1)
    | ')' | ']' | '}' as c ->
      (match stack with
       | top :: rest when top = matching c -> check rest (i + 1)
       | _ -> false)
    | _ -> check stack (i + 1)
  in
  check [] 0

let () =
  List.iter (fun s ->
    Printf.printf "%s: %b\n" s (is_balanced s)
  ) ["([]{})";"([)]";"((()))";"[{()}]";"("]
