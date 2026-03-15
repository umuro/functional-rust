(* 267. Infinite cycling with cycle() - OCaml *)

let cycle lst =
  let arr = Array.of_list lst in
  let n = Array.length arr in
  let i = ref 0 in
  Seq.forever (fun () -> let v = arr.(!i mod n) in incr i; v)

let () =
  let colors = ["red"; "green"; "blue"] in
  let cycled = cycle colors in
  let first9 = Seq.take 9 cycled |> List.of_seq in
  Printf.printf "Colors: %s\n" (String.concat ", " first9);

  let items = ["a"; "b"; "c"; "d"; "e"] in
  let roles = cycle ["leader"; "follower"] in
  let pairs = List.map2 (fun item role -> Printf.sprintf "%s->%s" item role)
    items (Seq.take 5 roles |> List.of_seq) in
  List.iter print_endline pairs
