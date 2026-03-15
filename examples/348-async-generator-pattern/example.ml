(* 348: Async Generator Pattern
   OCaml idiomatic generators use sequences (Seq.t) — lazy, pull-based,
   analogous to Rust's Iterator trait used as a generator. *)

(* A generator is just a mutable cursor over a list *)
type 'a generator = {
  items : 'a array;
  mutable index : int;
}

let make_generator items = { items = Array.of_list items; index = 0 }

let gen_next g =
  if g.index < Array.length g.items then begin
    let item = g.items.(g.index) in
    g.index <- g.index + 1;
    Some item
  end else
    None

let gen_reset g = g.index <- 0

(* Build a generator from an integer range [start, stop) *)
let range_generator start stop =
  make_generator (List.init (stop - start) (fun i -> start + i))

(* Convert generator to a Seq.t (idiomatic OCaml lazy sequence) *)
let gen_to_seq g =
  let rec step () =
    match gen_next g with
    | None      -> Seq.Nil
    | Some item -> Seq.Cons (item, step)
  in
  step

(* OCaml-idiomatic: pure lazy sequence generator *)
let range_seq start stop =
  Seq.unfold (fun i -> if i >= stop then None else Some (i, i + 1)) start

let () =
  (* Stateful generator test *)
  let g = make_generator [1;2;3] in
  assert (gen_next g = Some 1);
  assert (gen_next g = Some 2);
  assert (gen_next g = Some 3);
  assert (gen_next g = None);
  Printf.printf "generator yields 1,2,3 then None: ok\n%!";

  (* Reset test *)
  gen_reset g;
  assert (gen_next g = Some 1);
  Printf.printf "generator reset: ok\n%!";

  (* Range generator as seq *)
  let rg = range_generator 0 5 in
  let v = List.of_seq (gen_to_seq rg) in
  assert (v = [0;1;2;3;4]);
  Printf.printf "range_generator 0..5: %s\n%!"
    (v |> List.map string_of_int |> String.concat ", ");

  (* Pure lazy Seq approach *)
  let v2 = List.of_seq (range_seq 0 5) in
  assert (v2 = [0;1;2;3;4]);
  Printf.printf "range_seq 0..5: %s\n%!"
    (v2 |> List.map string_of_int |> String.concat ", ")
