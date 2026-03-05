(* Sequence — Windowed Operations *)
(* Sliding window over sequences *)

let windows n seq =
  let buf = Array.make n 0 in
  let i = ref 0 in
  seq |> Seq.filter_map (fun x ->
    buf.(!i mod n) <- x;
    incr i;
    if !i >= n then
      Some (Array.to_list (Array.init n (fun j -> buf.((!i - n + j) mod n))))
    else None
  )

let data = List.to_seq [1; 2; 3; 4; 5; 6; 7]
let wins = windows 3 data |> List.of_seq
let () = List.iter (fun w ->
  Printf.printf "[%s] " (String.concat "," (List.map string_of_int w))
) wins
