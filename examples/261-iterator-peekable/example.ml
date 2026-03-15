(* 261: Peekable iterator — inspect next element without consuming it.
   OCaml uses a simple ref-based wrapper or a custom sequence type.
   The idiomatic pattern is to carry the "peeked" value alongside the stream. *)

(* A peekable sequence: either we have a buffered element, or we pull from seq *)
type 'a peekable = {
  mutable buf : 'a option;
  mutable seq : 'a Seq.t;
}

let make_peekable seq = { buf = None; seq }

let peek p =
  match p.buf with
  | Some _ as v -> v
  | None ->
    match p.seq () with
    | Seq.Nil         -> None
    | Seq.Cons (x, rest) ->
      p.buf <- Some x;
      p.seq <- rest;
      Some x

let next p =
  match p.buf with
  | Some x -> p.buf <- None; Some x
  | None ->
    match p.seq () with
    | Seq.Nil         -> None
    | Seq.Cons (x, rest) -> p.seq <- rest; Some x

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* peek does not consume *)
  let p = make_peekable (List.to_seq [1; 2; 3]) in
  let peeked = peek p in
  let first  = next p in
  let second = next p in
  Printf.printf "peek=%s  next=%s  next=%s\n"
    (Option.fold ~none:"None" ~some:string_of_int peeked)
    (Option.fold ~none:"None" ~some:string_of_int first)
    (Option.fold ~none:"None" ~some:string_of_int second);

  (* Group consecutive equal elements using peek *)
  let data = [1; 1; 2; 3; 3] in
  let p2 = make_peekable (List.to_seq data) in
  let groups = ref [] in
  let continue_loop = ref true in
  while !continue_loop do
    match peek p2 with
    | None -> continue_loop := false
    | Some cur ->
      let group = ref [] in
      let inner = ref true in
      while !inner do
        match peek p2 with
        | Some v when v = cur ->
          ignore (next p2);
          group := v :: !group
        | _ -> inner := false
      done;
      groups := List.rev !group :: !groups
  done;
  let groups = List.rev !groups in
  Printf.printf "groups = [%s]\n"
    (groups |> List.map (fun g ->
       "[" ^ (g |> List.map string_of_int |> String.concat ";") ^ "]")
     |> String.concat ";");

  (* peek on empty *)
  let empty = make_peekable (List.to_seq []) in
  Printf.printf "peek empty = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (peek empty))
