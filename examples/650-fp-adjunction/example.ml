(* Adjunction in OCaml *)

(* Curry-Uncurry isomorphism from Product-Exponential adjunction *)
let curry f a b = f (a, b)
let uncurry f (a, b) = f a b

(* State monad from the adjunction *)
module State = struct
  type ('s, 'a) t = 's -> 'a * 's
  
  let pure a = fun s -> (a, s)
  
  let bind m f = fun s ->
    let (a, s') = m s in
    f a s'
    
  let get = fun s -> (s, s)
  
  let put s = fun _ -> ((), s)
  
  let run m s = m s
end

let () =
  (* Curry example *)
  let add (a, b) = a + b in
  let curried_add = curry add in
  let add_5 = curried_add 5 in
  Printf.printf "5 + 3 = %d\n" (add_5 3);
  
  (* State example *)
  let open State in
  let comp = bind get (fun x -> pure (x * 2)) in
  let (result, _) = run comp 21 in
  Printf.printf "Result: %d\n" result
