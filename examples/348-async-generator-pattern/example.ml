(* OCaml: generator via Seq (lazy sequence) *)

let fibonacci () : int Seq.t =
  let rec go a b () =
    Seq.Cons (a, go b (a + b))
  in go 0 1

let range_gen start stop step () =
  let rec go i () =
    if i >= stop then Seq.Nil
    else Seq.Cons (i, go (i + step))
  in go start ()

let () =
  (* Take first 10 Fibonacci numbers *)
  let fibs = fibonacci () |> Seq.take 10 |> List.of_seq in
  List.iter (Printf.printf "%d ") fibs;
  print_newline ();

  let r = range_gen 0 20 3 () |> List.of_seq in
  List.iter (Printf.printf "%d ") r;
  print_newline ()
