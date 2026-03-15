(* Example 209: Affine Traversal — At Most One Focus *)

(* An affine traversal focuses on at most one value.
   It's the combination of a prism (might not exist) and a lens (if it exists, it's unique).
   Think: "optional field accessor" *)

type ('s, 'a) affine = {
  preview : 's -> 'a option;
  set     : 'a -> 's -> 's;
}

(* Approach 1: Affine for optional record fields *)
type user = {
  name  : string;
  email : string option;
  phone : string option;
}

let email_affine : (user, string) affine = {
  preview = (fun u -> u.email);
  set = (fun e u -> { u with email = Some e });
}

let phone_affine : (user, string) affine = {
  preview = (fun u -> u.phone);
  set = (fun p u -> { u with phone = Some p });
}

(* Approach 2: Affine for map lookups *)
module StringMap = Map.Make(String)

let at_key (key : string) : (string StringMap.t, string) affine = {
  preview = (fun m -> StringMap.find_opt key m);
  set = (fun v m -> StringMap.add key v m);
}

(* Approach 3: Affine combinators *)
let over_affine (a : ('s, 'a) affine) (f : 'a -> 'a) (s : 's) : 's =
  match a.preview s with
  | Some v -> a.set (f v) s
  | None -> s

let compose_affine (outer : ('s, 'a) affine) (inner : ('a, 'b) affine) : ('s, 'b) affine = {
  preview = (fun s ->
    match outer.preview s with
    | Some a -> inner.preview a
    | None -> None);
  set = (fun b s ->
    match outer.preview s with
    | Some a -> outer.set (inner.set b a) s
    | None -> s);
}

(* === Tests === *)
let () =
  let user1 = { name = "Alice"; email = Some "alice@x.com"; phone = None } in
  let user2 = { name = "Bob"; email = None; phone = Some "555-1234" } in

  (* Preview *)
  assert (email_affine.preview user1 = Some "alice@x.com");
  assert (email_affine.preview user2 = None);
  assert (phone_affine.preview user2 = Some "555-1234");

  (* Set *)
  let u = email_affine.set "new@x.com" user1 in
  assert (u.email = Some "new@x.com");

  (* Over *)
  let u2 = over_affine email_affine String.uppercase_ascii user1 in
  assert (u2.email = Some "ALICE@X.COM");

  (* Over on missing field is no-op *)
  let u3 = over_affine email_affine String.uppercase_ascii user2 in
  assert (u3.email = None);

  (* Map lookup *)
  let m = StringMap.of_seq (List.to_seq [("a", "1"); ("b", "2")]) in
  let at_a = at_key "a" in
  assert (at_a.preview m = Some "1");
  assert ((at_key "z").preview m = None);

  let m2 = at_a.set "99" m in
  assert (at_a.preview m2 = Some "99");

  print_endline "✓ All tests passed"
