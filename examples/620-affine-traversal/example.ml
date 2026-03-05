(* Affine traversal in OCaml *)
(* An affine traversal is (preview, set) where set may not update *)

type ('s,'a) affine = {
  preview: 's -> 'a option;
  set:     'a -> 's -> 's;
}

(* Example: first element of a list *)
let head_affine = {
  preview = (function [] -> None | x::_ -> Some x);
  set     = (fun v -> function [] -> [] | _::t -> v::t);
}

(* Optional field: user's address city if present *)
type address = { city: string }
type user    = { name: string; address: address option }

let user_city = {
  preview = (fun u -> Option.map (fun a -> a.city) u.address);
  set     = (fun c u ->
    match u.address with
    | None   -> u
    | Some a -> { u with address = Some { a with city = c } }
  );
}

let () =
  let users = [
    { name="Alice"; address=Some{city="Boston"} };
    { name="Bob";   address=None };
  ] in
  List.iter (fun u ->
    match user_city.preview u with
    | Some c -> Printf.printf "%s lives in %s\n" u.name c
    | None   -> Printf.printf "%s has no address\n" u.name
  ) users;
  let u = List.hd users in
  let u2 = user_city.set "Cambridge" u in
  Printf.printf "moved to: %s\n" (Option.map (fun a->a.city) u2.address |> Option.get)
