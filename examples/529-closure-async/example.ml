(* Async closures in OCaml — using Lwt library concept *)
(* Standard OCaml doesn't have built-in async, this shows the pattern *)

(* Simulating async with continuation-passing style *)
type 'a promise = 'a  (* simplified *)

let async_compute f x = f x  (* would be Lwt.return (f x) in real Lwt *)
let async_map f promise = f promise

let () =
  (* Callback-style "async" *)
  let fetch url callback =
    (* simulate: in real code this would be async *)
    let result = Printf.sprintf "Data from %s" url in
    callback result
  in

  fetch "https://api.example.com/data" (fun data ->
    Printf.printf "Received: %s\n" data;
    fetch "https://api.example.com/more" (fun more ->
      Printf.printf "Also received: %s\n" more
    )
  )
