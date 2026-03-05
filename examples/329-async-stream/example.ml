(* OCaml: lazy sequences as stream analogy *)

type 'a stream = Empty | Cons of 'a * (unit -> 'a stream)

let rec take n = function
  | Empty -> []
  | Cons (x, rest) -> if n=0 then [] else x :: take (n-1) (rest ())

let range_stream start stop =
  let rec loop i () = if i>=stop then Empty else Cons(i, loop (i+1))
  in loop start ()

let map_stream f s =
  let rec go = function
    | Empty -> Empty
    | Cons (x, rest) -> Cons (f x, fun () -> go (rest ()))
  in go s

let filter_stream p s =
  let rec go = function
    | Empty -> Empty
    | Cons (x, rest) -> if p x then Cons(x, fun()->go(rest())) else go(rest())
  in go s

let () =
  let s = range_stream 0 20 |> filter_stream (fun x -> x mod 2 = 0) |> map_stream (fun x -> x*2) in
  List.iter (Printf.printf "%d ") (take 5 s); print_newline ()
