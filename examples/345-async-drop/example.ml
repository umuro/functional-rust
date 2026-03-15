(* OCaml: resource cleanup with finalizers *)

type connection = {
  id : int;
  mutable open_ : bool;
}

let open_connection id =
  Printf.printf "Opening connection %d\n" id;
  { id; open_ = true }

let close_connection conn =
  if conn.open_ then begin
    Printf.printf "Closing connection %d\n" conn.id;
    conn.open_ <- false
  end

let with_connection id f =
  let conn = open_connection id in
  let result = (try Ok (f conn) with e -> Error e) in
  close_connection conn;
  match result with
  | Ok v    -> v
  | Error e -> raise e

let () =
  with_connection 1 (fun conn ->
    Printf.printf "Using connection %d\n" conn.id;
    "done"
  ) |> ignore
