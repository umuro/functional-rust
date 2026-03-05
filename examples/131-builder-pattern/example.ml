(* Example 131: Builder Pattern with Typestate *)

(* Phantom types encode which required fields have been provided.
   The compiler rejects build() unless both 'name and 'email slots are Set. *)

type unset = Unset_t
type set = Set_t

type ('name, 'email) user_builder = {
  name : string option;
  email : string option;
  age : int option;
}

let empty_builder : (unset, unset) user_builder =
  { name = None; email = None; age = None }

(* Transitions first phantom from unset → set; email state preserved *)
let set_name name (b : (unset, 'e) user_builder) : (set, 'e) user_builder =
  { b with name = Some name }

(* Transitions second phantom from unset → set; name state preserved *)
let set_email email (b : ('n, unset) user_builder) : ('n, set) user_builder =
  { b with email = Some email }

(* Optional field — available in all states *)
let set_age age b = { b with age = Some age }

type user = {
  user_name : string;
  user_email : string;
  user_age : int option;
}

(* build only type-checks when both phantoms are Set *)
let build (b : (set, set) user_builder) : user =
  { user_name = Option.get b.name;
    user_email = Option.get b.email;
    user_age = b.age }

let () =
  let u1 = empty_builder |> set_name "Alice" |> set_email "alice@example.com" |> build in
  assert (u1.user_name = "Alice");
  assert (u1.user_email = "alice@example.com");
  assert (u1.user_age = None);

  let u2 = empty_builder
    |> set_email "bob@example.com"
    |> set_name "Bob"
    |> set_age 30
    |> build in
  assert (u2.user_name = "Bob");
  assert (u2.user_age = Some 30);

  (* This would be rejected by the type checker — set_name needs (unset, _):
     let _ = empty_builder |> set_name "x" |> set_name "y" |> ... *)

  print_endline "ok"
