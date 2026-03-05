(* 286. Creating iterators with from_fn() - OCaml *)
(* OCaml Seq.unfold is the equivalent *)

let () =
  (* Simple counter *)
  let counter =
    let n = ref 0 in
    Seq.unfold (fun () ->
      if !n >= 5 then None
      else begin incr n; Some (!n, ()) end
    ) ()
  in
  List.iter (fun x -> Printf.printf "%d " x) (List.of_seq counter);
  print_newline ();

  (* Fibonacci *)
  let fib =
    let a = ref 0 and b = ref 1 in
    Seq.forever (fun () ->
      let v = !a in
      let next = !a + !b in
      a := !b; b := next; v
    )
  in
  Printf.printf "Fib: %s\n"
    (String.concat ", " (List.map string_of_int (List.of_seq (Seq.take 10 fib))));

  (* Reading tokens from a string *)
  let words = ["hello"; "world"; "rust"] in
  let idx = ref 0 in
  let word_iter =
    Seq.unfold (fun () ->
      if !idx >= List.length words then None
      else begin
        let w = List.nth words !idx in
        incr idx;
        Some (w, ())
      end
    ) ()
  in
  List.iter (fun w -> Printf.printf "%s " w) (List.of_seq word_iter);
  print_newline ()
