(* List Comprehension via Bind *)
(* Simulate list comprehensions with monadic bind *)

let ( >>= ) lst f = List.concat_map f lst
let return x = [x]
let guard b = if b then [()] else []

(* Pythagorean triples *)
let triples n =
  List.init n (fun i -> i + 1) >>= fun a ->
  List.init n (fun i -> i + 1) >>= fun b ->
  List.init n (fun i -> i + 1) >>= fun c ->
  guard (a*a + b*b = c*c && a <= b) >>= fun () ->
  return (a, b, c)

let () = List.iter (fun (a,b,c) ->
  Printf.printf "(%d, %d, %d)\n" a b c
) (triples 20)
