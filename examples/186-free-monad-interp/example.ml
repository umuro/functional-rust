(* Free monad: build a DSL as a data structure, then interpret it.
   The program is pure; side effects only happen at interpretation. *)

(* DSL: a simple key-value store language *)
type 'a store_f =
  | Get : string * (string option -> 'a) -> 'a store_f
  | Put : string * string * 'a -> 'a store_f
  | Delete : string * 'a -> 'a store_f

(* Free monad *)
type 'a free =
  | Pure : 'a -> 'a free
  | Free : 'a free store_f -> 'a free

let pure x = Pure x

let get k f    = Free (Get (k, (fun v -> Pure (f v))))
let put k v a  = Free (Put (k, v, Pure a))
let delete k a = Free (Delete (k, Pure a))

let rec bind m f = match m with
  | Pure x -> f x
  | Free (Get (k, cont))    -> Free (Get (k, fun v -> bind (cont v) f))
  | Free (Put (k, v, next)) -> Free (Put (k, v, bind next f))
  | Free (Delete (k, next)) -> Free (Delete (k, bind next f))

(* Interpreter 1: in-memory hashtable *)
let run_memory tbl program =
  let rec go = function
    | Pure x -> x
    | Free (Get (k, cont))    -> go (cont (Hashtbl.find_opt tbl k))
    | Free (Put (k, v, next)) -> Hashtbl.replace tbl k v; go next
    | Free (Delete (k, next)) -> Hashtbl.remove tbl k; go next
  in go program

(* Interpreter 2: pure association list *)
let run_pure store program =
  let rec go store = function
    | Pure x -> (x, store)
    | Free (Get (k, cont))    -> go store (cont (List.assoc_opt k store))
    | Free (Put (k, v, next)) -> go ((k,v) :: List.filter (fun (k',_) -> k' <> k) store) next
    | Free (Delete (k, next)) -> go (List.filter (fun (k',_) -> k' <> k) store) next
  in go store program

let () =
  (* Build the program once *)
  let program =
    bind (put "name" "Alice" ()) (fun () ->
    bind (put "age"  "30"    ()) (fun () ->
    bind (get "name" (fun v -> v)) (fun name ->
    pure name)))
  in

  (* Run with two different interpreters *)
  let tbl = Hashtbl.create 4 in
  let result1 = run_memory tbl program in
  Printf.printf "memory interp: %s\n" (Option.value result1 ~default:"none");

  let (result2, _) = run_pure [] program in
  Printf.printf "pure interp:   %s\n" (Option.value result2 ~default:"none")
