(* 489. Arc<str> concept – OCaml *)
(* OCaml GC handles thread-safe sharing; demonstrate with domains *)
let shared_name = "global shared string"

let () =
  let domains = Array.init 4 (fun id ->
    Domain.spawn (fun () ->
      Printf.printf "domain %d: %s (ptr=%d)\n"
        id shared_name (Obj.repr shared_name |> Obj.obj)
    )
  ) in
  Array.iter Domain.join domains
