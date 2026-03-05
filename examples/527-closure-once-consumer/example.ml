(* FnOnce analog in OCaml — using a boolean flag to enforce once *)
let make_one_shot f =
  let used = ref false in
  fun () ->
    if !used then failwith "already consumed"
    else begin used := true; f () end

let () =
  let greet = make_one_shot (fun () -> Printf.printf "Hello, once!\n") in
  greet ();
  (try greet () with Failure msg -> Printf.printf "Error: %s\n" msg);

  (* Transfer ownership of a resource *)
  let buffer = Buffer.create 64 in
  Buffer.add_string buffer "some data";
  let consume_buffer = make_one_shot (fun () ->
    let contents = Buffer.contents buffer in
    Buffer.clear buffer;
    contents
  ) in
  Printf.printf "Contents: %s\n" (consume_buffer ())
