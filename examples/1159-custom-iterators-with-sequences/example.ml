(* Custom Iterators with Sequences *)
(* Range and step iterators using Seq *)

let range ?(step=1) start stop =
  Seq.unfold (fun i ->
    if (step > 0 && i < stop) || (step < 0 && i > stop)
    then Some (i, i + step)
    else None
  ) start

let () =
  range 0 10 |> Seq.iter (fun x -> Printf.printf "%d " x);
  print_newline ();
  range ~step:2 0 20 |> Seq.iter (fun x -> Printf.printf "%d " x);
  print_newline ();
  range ~step:(-3) 30 0 |> Seq.iter (fun x -> Printf.printf "%d " x);
  print_newline ()
