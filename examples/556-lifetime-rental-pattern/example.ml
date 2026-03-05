(* Rental / self-referential pattern in OCaml *)
(* OCaml: GC makes this trivial -- values reference each other freely *)

type parsed = {
  source: string;
  tokens: string list;
  word_count: int;
}

let parse text =
  let tokens = String.split_on_char ' ' text in
  { source = text; tokens; word_count = List.length tokens }

let () =
  let p = parse "hello world rust programming" in
  Printf.printf "source: %s\n" p.source;
  Printf.printf "words: %d\n" p.word_count;
  List.iter (fun t -> Printf.printf "  token: %s\n" t) p.tokens
