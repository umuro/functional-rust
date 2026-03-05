(* Forth Interpreter *)
(* Stack-based language interpreter with user-defined words *)

type forth_state = {
  stack : int list;
  defs : (string * string list) list;
}

let empty = { stack = []; defs = [] }

let lookup word state =
  List.assoc_opt (String.uppercase_ascii word) state.defs

let rec eval_word word state =
  match lookup word state with
  | Some expansion -> eval_words expansion state
  | None ->
    match int_of_string_opt word with
    | Some n -> Ok { state with stack = n :: state.stack }
    | None ->
      match String.uppercase_ascii word, state.stack with
      | "+", a :: b :: rest -> Ok { state with stack = (b + a) :: rest }
      | "-", a :: b :: rest -> Ok { state with stack = (b - a) :: rest }
      | "*", a :: b :: rest -> Ok { state with stack = (b * a) :: rest }
      | "/", 0 :: _ :: _ -> Error "divide by zero"
      | "/", a :: b :: rest -> Ok { state with stack = (b / a) :: rest }
      | "DUP", a :: rest -> Ok { state with stack = a :: a :: rest }
      | "DROP", _ :: rest -> Ok { state with stack = rest }
      | "SWAP", a :: b :: rest -> Ok { state with stack = b :: a :: rest }
      | "OVER", a :: b :: rest -> Ok { state with stack = b :: a :: b :: rest }
      | _ -> Error ("unknown word: " ^ word)

and eval_words words state =
  List.fold_left (fun acc w ->
    match acc with Error _ -> acc | Ok s -> eval_word w s
  ) (Ok state) words

let eval_line line state =
  let words = String.split_on_char ' ' line in
  match words with
  | ":" :: name :: rest when List.length rest > 0 ->
    let body = List.rev (List.tl (List.rev rest)) in
    if int_of_string_opt name <> None then Error "cannot redefine numbers"
    else
      (* Expand existing definitions in body *)
      let expanded = List.concat_map (fun w ->
        match lookup w state with Some exp -> exp | None -> [w]
      ) body in
      Ok { state with defs = (String.uppercase_ascii name, expanded) :: state.defs }
  | _ -> eval_words words state

let evaluate lines =
  let result = List.fold_left (fun acc line ->
    match acc with Error _ -> acc | Ok s -> eval_line line s
  ) (Ok empty) lines in
  match result with
  | Error e -> Error e
  | Ok state -> Ok (List.rev state.stack)
