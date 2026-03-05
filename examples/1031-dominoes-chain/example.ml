(* Dominoes Chain *)
(* Backtracking search for Eulerian path in domino chain *)

type dominoe = int * int

let chain = function
  | [] -> Some []
  | first :: rest ->
    let rec go stones path =
      match stones with
      | [] ->
        let (a, _) = List.hd path and (_, b) = List.hd (List.rev path) in
        if a = b then Some (List.rev path) else None
      | _ ->
        let right_end = snd (List.hd path) in
        let rec try_each before = function
          | [] -> None
          | (a, b) :: after ->
            let remaining = List.rev_append before after in
            let result =
              if a = right_end then go remaining ((a, b) :: path)
              else if b = right_end then go remaining ((b, a) :: path)
              else None
            in
            match result with
            | Some _ -> result
            | None -> try_each ((a, b) :: before) after
        in
        try_each [] stones
    in
    go rest [first]
