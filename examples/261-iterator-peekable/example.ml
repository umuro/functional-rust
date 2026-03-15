(* 261. Lookahead with Peekable - OCaml *)

type 'a peekable = {
  mutable peeked: 'a option;
  mutable rest: 'a list;
}

let make_peekable lst = { peeked = None; rest = lst }

let peek p =
  match p.peeked with
  | Some _ as v -> v
  | None ->
    (match p.rest with
     | [] -> None
     | x :: _ -> p.peeked <- Some x; Some x)

let next p =
  match p.peeked with
  | Some v -> p.peeked <- None; (match p.rest with _ :: xs -> p.rest <- xs | [] -> ()); Some v
  | None ->
    match p.rest with
    | [] -> None
    | x :: xs -> p.rest <- xs; Some x

let () =
  let p = make_peekable [1; 2; 3; 4; 5] in
  Printf.printf "Peek: %s\n" (match peek p with Some n -> string_of_int n | None -> "None");
  Printf.printf "Next: %s\n" (match next p with Some n -> string_of_int n | None -> "None");
  Printf.printf "Next: %s\n" (match next p with Some n -> string_of_int n | None -> "None");
  let p2 = make_peekable [1; 1; 2; 2; 3] in
  let rec group_eq () =
    match next p2 with
    | None -> []
    | Some x ->
      let rec collect acc =
        match peek p2 with
        | Some y when y = x -> ignore (next p2); collect (x :: acc)
        | _ -> List.rev (x :: acc)
      in
      collect [] :: group_eq ()
  in
  let groups = group_eq () in
  List.iter (fun g ->
    Printf.printf "[%s] " (String.concat ";" (List.map string_of_int g))
  ) groups;
  print_newline ()
