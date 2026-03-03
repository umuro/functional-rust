effect Exn : string -> 'a

let try_with f handler =
  match f () with
  | v -> v
  | effect (Exn msg) _ -> handler msg

let safe_div a b =
  if b = 0 then perform (Exn "division by zero")
  else a / b

let () =
  let r1 = try_with (fun () -> safe_div 10 2) (fun _ -> -1) in
  Printf.printf "10/2 = %d\n" r1;
  let r2 = try_with (fun () -> safe_div 10 0) (fun msg -> Printf.printf "caught: %s\n" msg; -1) in
  Printf.printf "10/0 handled = %d\n" r2
