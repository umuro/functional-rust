(* Example 055: Option Monad *)
(* Monadic bind (>>=) for Option: chain computations that may fail *)

let return_ x = Some x
let bind m f = match m with None -> None | Some x -> f x
let ( >>= ) = bind

(* Approach 1: Safe dictionary lookup chain *)
let lookup key assoc = List.assoc_opt key assoc

let env = [("HOME", "/home/user"); ("USER", "alice")]
let paths = [("/home/user", ["documents"; "photos"])]

let find_user_docs () =
  lookup "HOME" env >>= fun home ->
  lookup home paths >>= fun dirs ->
  if List.mem "documents" dirs then Some "documents found"
  else None

(* Approach 2: Safe arithmetic chain *)
let safe_div x y = if y = 0 then None else Some (x / y)
let safe_sqrt x = if x < 0 then None else Some (Float.sqrt (Float.of_int x))

let compute a b =
  safe_div a b >>= fun q ->
  safe_sqrt q >>= fun r ->
  Some (Float.to_int r)

(* Approach 3: Using Option.bind from stdlib *)
let compute_stdlib a b =
  Option.bind (safe_div a b) (fun q ->
  Option.bind (safe_sqrt q) (fun r ->
  Some (Float.to_int r)))

let () =
  (* Lookup chain *)
  assert (find_user_docs () = Some "documents found");
  assert (lookup "MISSING" env >>= fun _ -> Some "x" = None);

  (* Arithmetic chain *)
  assert (compute 100 4 = Some 5);
  assert (compute 100 0 = None);  (* div by zero *)
  assert (compute (-100) 1 = None);  (* negative sqrt *)

  (* Stdlib version same results *)
  assert (compute_stdlib 100 4 = Some 5);
  assert (compute_stdlib 100 0 = None);

  Printf.printf "✓ All tests passed\n"
