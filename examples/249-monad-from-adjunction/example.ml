(* Every adjunction F ⊣ G yields a monad G . F.
   The curry/uncurry adjunction yields the State monad.
   F = (- × S), G = (S → -), G.F = S → (- × S) = State S *)

(* State monad derived from the product-exponential adjunction *)
type ('s, 'a) state = State of ('s -> 'a * 's)

let run (State f) s = f s

(* unit of monad = unit of adjunction *)
let return x = State (fun s -> (x, s))

(* bind = counit . G(eps) where eps is counit of adjunction *)
let bind (State f) k =
  State (fun s ->
    let (a, s') = f s in
    let (State g) = k a in
    g s')

let get       = State (fun s -> (s, s))
let put s     = State (fun _ -> ((), s))
let modify f  = State (fun s -> ((), f s))

let () =
  let program =
    bind get (fun n ->
    bind (put (n + 10)) (fun () ->
    bind get (fun m ->
    bind (modify (fun s -> s * 2)) (fun () ->
    bind get (fun final ->
    return (n, m, final))))))
  in
  let ((a, b, c), s) = run program 5 in
  Printf.printf "initial=%d, after+10=%d, after*2=%d, final_state=%d\n" a b c s;
  Printf.printf "State monad from product-exponential adjunction\n"
