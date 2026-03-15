(* 094: Peekable Iterator
   OCaml: look-ahead via Seq or by keeping a "pending" option *)

(* --- A peekable wrapper over a Seq --- *)

(* The cleanest approach: just use Seq directly, which is already "peekable"
   because forcing it gives you the head without consuming it. *)

(* dedup: remove consecutive duplicates — the classic peekable use-case *)
let dedup xs =
  (* compare adjacent elements: fold while tracking last seen *)
  let rec aux acc prev = function
    | [] -> List.rev acc
    | x :: rest ->
      if x = prev then aux acc prev rest
      else aux (x :: acc) x rest
  in
  match xs with
  | []      -> []
  | x :: rest -> aux [x] x rest

(* --- Generic dedup_by with a comparator --- *)
let dedup_by eq xs =
  let rec aux acc prev = function
    | [] -> List.rev acc
    | x :: rest ->
      if eq x prev then aux acc prev rest
      else aux (x :: acc) x rest
  in
  match xs with
  | []      -> []
  | x :: rest -> aux [x] x rest

(* --- Peekable generator type for demonstration --- *)

type 'a peekable = {
  mutable buf  : 'a option;    (* peeked value, if any *)
  mutable inner: 'a Seq.t;
}

let make_peekable seq = { buf = None; inner = seq }

let peek p =
  match p.buf with
  | Some _ as v -> v
  | None ->
    match p.inner () with
    | Seq.Nil -> None
    | Seq.Cons (x, rest) ->
      p.buf <- Some x;
      p.inner <- rest;
      Some x

let next p =
  match p.buf with
  | Some x -> p.buf <- None; Some x
  | None ->
    match p.inner () with
    | Seq.Nil -> None
    | Seq.Cons (x, rest) -> p.inner <- rest; Some x

let () =
  Printf.printf "dedup [1;1;2;2;2;3;3;1] = [%s]\n"
    (String.concat "; " (List.map string_of_int (dedup [1;1;2;2;2;3;3;1])));
  Printf.printf "dedup [] = [%s]\n"
    (String.concat "; " (List.map string_of_int (dedup [])));
  Printf.printf "dedup [5] = [%s]\n"
    (String.concat "; " (List.map string_of_int (dedup [5])));

  (* peekable generator *)
  let p = make_peekable (List.to_seq [1;2;3]) in
  Printf.printf "peek  = %s\n" (match peek p with Some v -> string_of_int v | None -> "None");
  Printf.printf "next  = %s\n" (match next p with Some v -> string_of_int v | None -> "None");
  Printf.printf "peek2 = %s\n" (match peek p with Some v -> string_of_int v | None -> "None")
