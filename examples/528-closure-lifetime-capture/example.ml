(* Closures capturing references in OCaml — GC manages lifetimes *)
let make_length_checker s =
  (* s is captured by GC reference — lives as long as closure *)
  fun () -> String.length s

let () =
  let len =
    let s = "hello world" in
    make_length_checker s  (* s kept alive by closure *)
  in
  Printf.printf "length: %d\n" (len ())
