(* 094: Peekable Iterator *)

(* OCaml: manual peekable wrapper *)
type 'a peekable = { mutable peeked: 'a option; seq: 'a Seq.t ref }

let make_peekable s = { peeked = None; seq = ref s }

let peek p =
  match p.peeked with
  | Some _ as v -> v
  | None ->
    match !(p.seq) () with
    | Seq.Nil -> None
    | Seq.Cons (x, rest) -> p.peeked <- Some x; p.seq := rest; Some x

let next p =
  match p.peeked with
  | Some x -> p.peeked <- None; Some x
  | None ->
    match !(p.seq) () with
    | Seq.Nil -> None
    | Seq.Cons (x, rest) -> p.seq := rest; Some x

(* Use: skip consecutive duplicates *)
let dedup s =
  let p = make_peekable s in
  let rec aux acc =
    match next p with
    | None -> List.rev acc
    | Some x ->
      (* skip while peek = x *)
      let rec skip () = match peek p with Some y when y = x -> ignore (next p); skip () | _ -> () in
      skip (); aux (x :: acc)
  in
  aux []

(* Tests *)
let () =
  let s = List.to_seq [1;1;2;2;2;3;3;1] in
  assert (dedup s = [1;2;3;1]);
  assert (dedup (List.to_seq []) = []);
  assert (dedup (List.to_seq [5]) = [5]);
  Printf.printf "✓ All tests passed\n"
