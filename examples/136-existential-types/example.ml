(* Example 136: Existential Types *)

(* Approach 1: Existential via first-class modules *)
module type SHOWABLE = sig
  type t
  val value : t
  val show : t -> string
end

let pack_showable (type a) (show : a -> string) (value : a) : (module SHOWABLE) =
  (module struct
    type t = a
    let value = value
    let show = show
  end)

let show_it (m : (module SHOWABLE)) =
  let module M = (val m) in M.show M.value

(* Approach 2: Existential via closure/record *)
type showable = { show : unit -> string }

let make_showable show value = { show = fun () -> show value }

(* Approach 3: GADT existential *)
type any_list = AnyList : 'a list * ('a -> string) -> any_list

let show_any_list (AnyList (lst, show)) =
  String.concat ", " (List.map show lst)

(* Tests *)
let () =
  let items = [
    pack_showable string_of_int 42;
    pack_showable (fun s -> s) "hello";
    pack_showable string_of_float 3.14;
  ] in
  let results = List.map show_it items in
  assert (results = ["42"; "hello"; "3.14"]);

  let items2 = [
    make_showable string_of_int 42;
    make_showable (fun s -> s) "hello";
  ] in
  assert ((List.hd items2).show () = "42");

  let al = AnyList ([1;2;3], string_of_int) in
  assert (show_any_list al = "1, 2, 3");

  Printf.printf "✓ All tests passed\n"
