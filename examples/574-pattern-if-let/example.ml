(* OCaml single-arm match simulation *)
let () =
  let v = Some 42 in
  (match v with Some n -> Printf.printf "got %d\n" n | None -> ());

  let q = Queue.create () in
  List.iter (fun x -> Queue.add x q) [1;2;3;4;5];
  while not (Queue.is_empty q) do
    Printf.printf "pop %d\n" (Queue.pop q)
  done
