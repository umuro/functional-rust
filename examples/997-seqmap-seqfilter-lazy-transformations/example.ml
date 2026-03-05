(* Seq.map, Seq.filter — Lazy Transformations *)
(* Transform and filter sequences lazily *)

let naturals = Seq.unfold (fun n -> Some (n, n + 1)) 1

let even_squares =
  naturals
  |> Seq.map (fun n -> n * n)
  |> Seq.filter (fun n -> n mod 2 = 0)
  |> Seq.take 8
  |> List.of_seq

let () = List.iter (fun x -> Printf.printf "%d " x) even_squares
(* Output: 4 16 36 64 100 144 196 256 *)
