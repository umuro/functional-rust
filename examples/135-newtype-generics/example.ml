(* Example 135: Generic Newtype Patterns *)

(* Approach 1: Private type aliases *)
type email = Email of string
type username = Username of string
type user_id = UserId of int

let email_of_string s =
  if String.contains s '@' then Some (Email s) else None

let string_of_email (Email s) = s

let username_of_string s =
  if String.length s >= 3 then Some (Username s) else None

let string_of_username (Username s) = s

(* Approach 2: Functor-based generic wrapper *)
module type WRAPPER = sig
  type inner
  type t
  val wrap : inner -> t
  val unwrap : t -> inner
end

module MakeWrapper (I : sig type t end) : WRAPPER with type inner = I.t = struct
  type inner = I.t
  type t = inner
  let wrap x = x
  let unwrap x = x
end

module PositiveInt = struct
  type t = int
  let create n = if n > 0 then Some n else None
  let value x = x
end

(* Approach 3: Sorted list newtype *)
module SortedList = struct
  type 'a t = 'a list  (* invariant: always sorted *)
  let empty = []
  let of_list lst = List.sort compare lst
  let insert x lst = List.sort compare (x :: lst)
  let to_list t = t
  let min = function [] -> None | x :: _ -> Some x
end

(* Tests *)
let () =
  assert (email_of_string "test@example.com" <> None);
  assert (email_of_string "invalid" = None);
  assert (string_of_email (Email "a@b.com") = "a@b.com");
  assert (username_of_string "ab" = None);
  assert (username_of_string "abc" <> None);
  assert (PositiveInt.create 5 = Some 5);
  assert (PositiveInt.create (-1) = None);
  let sl = SortedList.of_list [3;1;4;1;5] in
  assert (SortedList.to_list sl = [1;1;3;4;5]);
  assert (SortedList.min sl = Some 1);
  Printf.printf "✓ All tests passed\n"
