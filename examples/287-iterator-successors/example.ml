(* 287. Recursive sequences with successors() - OCaml *)

let successors first f =
  Seq.unfold (fun state ->
    match state with
    | None -> None
    | Some x -> Some (x, f x)
  ) (Some first)

let () =
  (* Powers of 2 *)
  let powers_of_2 = successors 1 (fun x -> if x < 256 then Some (x * 2) else None) in
  Printf.printf "Powers of 2: %s\n"
    (String.concat ", " (List.map string_of_int (List.of_seq powers_of_2)));

  (* Collatz sequence *)
  let collatz n =
    successors n (fun x ->
      if x = 1 then None
      else if x mod 2 = 0 then Some (x / 2)
      else Some (3 * x + 1)
    )
  in
  Printf.printf "Collatz(6): %s\n"
    (String.concat ", " (List.map string_of_int (List.of_seq (collatz 6))));

  (* Geometric sequence *)
  let geo = successors 1.0 (fun x -> if x > 100.0 then None else Some (x *. 3.0)) in
  Printf.printf "Geometric: %s\n"
    (String.concat ", " (List.map (Printf.sprintf "%.0f") (List.of_seq geo)))
