(* Delimited continuations: capture only part of the call stack.
   OCaml 5 effects give us delimited continuations naturally. *)
effect Shift : ('a -> 'b) -> 'a

let reset f =
  match f () with
  | v -> v
  | effect (Shift k) handler -> handler k

let shift f = perform (Shift f)

(* Classic: list of successes (nondeterminism) *)
let choose lst =
  shift (fun k -> List.concat_map k lst)

let () =
  (* Pythagorean triples up to 10 *)
  let triples =
    reset (fun () ->
      let a = choose [1;2;3;4;5;6;7;8;9;10] in
      let b = choose [a;a+1;a+2;a+3;a+4;a+5;a+6;a+7;a+8;a+9;a+10] in
      let c = choose [b;b+1;b+2;b+3;b+4;b+5] in
      if a*a + b*b = c*c then [(a,b,c)] else [])
  in
  List.iter (fun (a,b,c) -> Printf.printf "(%d,%d,%d)\n" a b c) (List.filteri (fun i _ -> i < 5) triples)
