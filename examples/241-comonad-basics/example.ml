(* Comonads are the dual of monads.
   Monad:   return :: a -> m a    bind :: m a -> (a -> m b) -> m b
   Comonad: extract :: w a -> a   extend :: w a -> (w a -> b) -> w b *)

(* The simplest comonad: Identity *)
type 'a identity = Id of 'a

let extract_id (Id x) = x
let extend_id (Id x) f = Id (f (Id x))
let duplicate_id (Id x) = Id (Id x)

(* Non-empty list (zipper) comonad: focus on current element *)
type 'a zipper = {
  left  : 'a list;   (* elements to the left, reversed *)
  focus : 'a;        (* current focus *)
  right : 'a list;   (* elements to the right *)
}

let make_zipper lst = match lst with
  | []     -> failwith "empty"
  | x :: xs -> { left = []; focus = x; right = xs }

let extract z = z.focus

let move_left z = match z.left with
  | []     -> None
  | x :: xs -> Some { left = xs; focus = x; right = z.focus :: z.right }

let move_right z = match z.right with
  | []     -> None
  | x :: xs -> Some { left = z.focus :: z.left; focus = x; right = xs }

let extend z f =
  (* Apply f to every possible focus position *)
  let rec go_left z acc =
    let v = f z in
    match move_left z with
    | None -> (v, acc)
    | Some z' -> go_left z' (v :: acc)
  in
  let rec go_right z acc =
    let v = f z in
    match move_right z with
    | None -> (v, acc)
    | Some z' -> go_right z' (acc @ [v])
  in
  let (center, lefts) = go_left z [] in
  let (_, rights) = go_right z [] in
  { left = lefts; focus = center; right = List.tl rights }

let () =
  let z = make_zipper [1; 2; 3; 4; 5] in
  Printf.printf "focus = %d\n" (extract z);

  let z2 = Option.get (move_right z) in
  Printf.printf "after move_right, focus = %d\n" (extract z2);

  (* Comonad law: extract . extend f = f *)
  let f w = w.focus * 2 in
  let extended = extend z f in
  assert (extract extended = f z);
  Printf.printf "extract . extend f = f (law holds)\n"
