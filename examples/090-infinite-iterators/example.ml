(* 090: Infinite Iterators — cycle, repeat, from_fn
   OCaml's Seq module provides cycle, repeat, and from_fn equivalents *)

(* --- Approach 1: Cycle a sequence --- *)

(* Seq.cycle is available in OCaml 5.1+; here's a simple version *)
let rec cycle xs =
  if xs = [] then Seq.empty
  else
    let rec go = function
      | []     -> go xs           (* restart *)
      | h :: t -> fun () -> Seq.Cons (h, go t)
    in
    go xs

(* --- Approach 2: Repeat a single value --- *)

let repeat x = Seq.repeat x    (* OCaml 4.14+ *)

(* Explicit version: *)
let repeat_explicit x =
  Seq.unfold (fun () -> Some (x, ())) ()

(* --- Approach 3: from_fn equivalent — Seq.of_dispenser --- *)

(* A "dispenser" is a unit -> 'a option function; Seq.of_dispenser wraps it *)
let from_fn f = Seq.of_dispenser f

(* --- Approach 4: repeat_with (stateful generator) --- *)

let repeat_with f =
  Seq.of_dispenser (fun () -> Some (f ()))

let () =
  (* cycle [1;2;3], take 7 *)
  Printf.printf "cycle [1;2;3] take 7 = [%s]\n"
    (String.concat "; "
      (List.map string_of_int (List.of_seq (Seq.take 7 (cycle [1;2;3])))));

  (* repeat 42, take 4 *)
  Printf.printf "repeat 42 take 4 = [%s]\n"
    (String.concat "; "
      (List.map string_of_int (List.of_seq (Seq.take 4 (repeat 42)))));

  (* from_fn: naturals 0..4 *)
  let n = ref 0 in
  let v = from_fn (fun () -> let x = !n in incr n; if x >= 5 then None else Some x) in
  Printf.printf "from_fn 0..4 = [%s]\n"
    (String.concat "; " (List.map string_of_int (List.of_seq v)));

  (* repeat_with: squares 1,4,9,16 *)
  let c = ref 0 in
  let squares = repeat_with (fun () -> incr c; !c * !c) in
  Printf.printf "repeat_with squares take 4 = [%s]\n"
    (String.concat "; "
      (List.map string_of_int (List.of_seq (Seq.take 4 squares))))
