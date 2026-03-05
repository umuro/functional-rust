(* 283. ExactSizeIterator for known-length - OCaml *)
(* OCaml arrays always know their length *)

let () =
  let arr = [|1; 2; 3; 4; 5|] in
  Printf.printf "Length: %d\n" (Array.length arr);

  (* Pre-allocate based on known length *)
  let src = Array.init 100 (fun i -> i * i) in
  let target = Array.make (Array.length src) 0 in
  Array.blit src 0 target 0 (Array.length src);
  Printf.printf "Copied %d elements\n" (Array.length target);

  (* Size hint for lists *)
  let lst = List.init 5 (fun i -> i + 1) in
  Printf.printf "List length: %d\n" (List.length lst);

  (* Simulate size_hint *)
  Printf.printf "Expected items: 5\n"
