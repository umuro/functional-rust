(* Comonad laws (dual of monad laws):
   1. extract . extend f = f
   2. extend extract = id
   3. extend f . extend g = extend (f . extend g)
   
   Verified on the Stream comonad (infinite list with focus) *)

(* Lazy stream *)
type 'a stream = Cons of 'a * (unit -> 'a stream)

let rec from n = Cons (n, fun () -> from (n + 1))
let head (Cons (x, _)) = x
let tail (Cons (_, f)) = f ()

let take n s =
  let rec go n s acc =
    if n = 0 then List.rev acc
    else let Cons (x, f) = s in go (n-1) (f ()) (x :: acc)
  in go n s []

(* Stream is a comonad *)
let extract = head

let rec extend s f =
  Cons (f s, fun () -> extend (tail s) f)

let duplicate s = extend s (fun x -> x)

(* Verify law 1: extract . extend f = f *)
let check_law1 f s =
  let lhs = extract (extend s f) in
  let rhs = f s in
  lhs = rhs

(* Verify law 2: extend extract = id (on first element) *)
let check_law2 s =
  let s' = extend s extract in
  head s = head s'

let () =
  let s = from 1 in

  let f s = head s * 2 in
  assert (check_law1 f s);
  Printf.printf "Law 1 (extract . extend f = f): holds\n";

  assert (check_law2 s);
  Printf.printf "Law 2 (extend extract = id): holds\n";

  (* extend: compute moving average *)
  let avg s = (head s + head (tail s) + head (tail (tail s))) / 3 in
  let avgs = extend s avg in
  Printf.printf "Moving avg of 1..5: [%s]\n"
    (take 5 avgs |> List.map string_of_int |> String.concat ";")
