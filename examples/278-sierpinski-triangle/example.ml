let sierpinski n =
  let rec go n =
    if n = 0 then ["*"]
    else
      let prev = go (n - 1) in
      let width = 1 lsl n - 1 in (* 2^n - 1 *)
      let pad s = String.make ((width - String.length s) / 2) ' ' ^ s in
      let top = List.map pad prev in
      let bottom = List.map (fun s -> s ^ " " ^ s) prev in
      top @ bottom
  in
  List.iter print_endline (go n)

let () =
  sierpinski 4;
  (* Basic assertions *)
  let lines n =
    let rec go n =
      if n = 0 then ["*"]
      else
        let prev = go (n - 1) in
        let width = 1 lsl n - 1 in
        let pad s = String.make ((width - String.length s) / 2) ' ' ^ s in
        let top = List.map pad prev in
        let bottom = List.map (fun s -> s ^ " " ^ s) prev in
        top @ bottom
    in go n
  in
  assert (List.length (lines 0) = 1);
  assert (List.length (lines 4) = 16);
  print_endline "ok"
