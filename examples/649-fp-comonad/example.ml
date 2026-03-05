(* Comonad in OCaml *)

module type COMONAD = sig
  type 'a t
  val extract : 'a t -> 'a
  val extend : ('a t -> 'b) -> 'a t -> 'b t
  val duplicate : 'a t -> 'a t t
end

(* Non-empty list comonad *)
type 'a nonempty = { head : 'a; tail : 'a list }

let extract ne = ne.head

let rec tails = function
  | { head; tail = [] } -> [{ head; tail = [] }]
  | { head; tail = t :: ts } -> 
      { head; tail = t :: ts } :: tails { head = t; tail = ts }

let extend f ne = 
  match tails ne with
  | [] -> failwith "impossible"
  | h :: t -> { head = f h; tail = List.map f t }

let duplicate ne = extend Fun.id ne

let () =
  let ne = { head = 1; tail = [2; 3; 4] } in
  let doubled = extend (fun n -> n.head * 2) ne in
  Printf.printf "Head doubled: %d\n" doubled.head
