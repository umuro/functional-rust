effect Get : int
effect Put : int -> unit

let run_state init f =
  let state = ref init in
  match f () with
  | v -> (v, !state)
  | effect Get k -> continue k !state
  | effect (Put n) k -> state := n; continue k ()

let () =
  let (result, final) = run_state 0 (fun () ->
    let n = perform Get in
    perform (Put (n + 10));
    let m = perform Get in
    perform (Put (m * 2));
    perform Get
  ) in
  Printf.printf "result=%d final_state=%d\n" result final
