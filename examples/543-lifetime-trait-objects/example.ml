(* Trait objects / dynamic dispatch in OCaml via polymorphic variants or first-class modules *)

module type Renderer = sig
  val render: unit -> string
end

(* Dynamic dispatch via first-class module *)
let make_renderer (module R : Renderer) = (module R : Renderer)

module Hello = struct
  let render () = "Hello, World!"
end

module Farewell = struct
  let render () = "Goodbye!"
end

let () =
  let renderers : (module Renderer) list = [
    (module Hello);
    (module Farewell);
  ] in
  List.iter (fun (module R : Renderer) ->
    Printf.printf "%s\n" (R.render ())
  ) renderers
