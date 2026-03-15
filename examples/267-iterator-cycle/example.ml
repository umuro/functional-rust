(* 267: cycle — repeat a finite sequence infinitely.
   OCaml: Seq.cycle (5.1+) or hand-rolled using recursive Seq. *)

(* cycle: repeat a list infinitely as a lazy Seq *)
let cycle lst =
  if lst = [] then Seq.empty
  else
    let arr = Array.of_list lst in
    let n = Array.length arr in
    Seq.unfold (fun i -> Some (arr.(i mod n), i + 1)) 0

let () =
  (* Basic cycle: take 7 elements from cycling [1;2;3] *)
  let r1 = cycle [1;2;3] |> Seq.take 7 |> List.of_seq in
  Printf.printf "cycle [1;2;3] take 7  = [%s]\n"
    (r1 |> List.map string_of_int |> String.concat ";");

  (* Round-robin: zip a list with a cycling label sequence *)
  let items = [1;2;3;4] in
  let labels = cycle ["a"; "b"] in
  let paired = Seq.zip (List.to_seq items) labels |> List.of_seq in
  Printf.printf "zip with cycling labels:\n";
  List.iter (fun (n, l) -> Printf.printf "  (%d, %s)\n" n l) paired;
  (* Check that item 3 (index 2) gets label "a" again *)
  Printf.printf "index 2 label = %s\n" (snd (List.nth paired 2));

  (* Alternating booleans *)
  let alt = cycle [true; false] |> Seq.take 6 |> List.of_seq in
  Printf.printf "alternating bools = [%s]\n"
    (alt |> List.map string_of_bool |> String.concat ";");

  (* Cycle an empty list yields empty seq *)
  let empty_cycle = cycle [] |> Seq.take 5 |> List.of_seq in
  Printf.printf "cycle [] take 5 = [%s]\n"
    (empty_cycle |> List.map string_of_int |> String.concat ";")
