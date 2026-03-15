(* 742: Type witnesses — evidence passing for invariants in OCaml *)
(* A type witness is a value that carries evidence of a type-level invariant.
   Creating the witness IS the proof; the type system propagates the guarantee
   to all callers without repeated runtime checks. *)

(* ── Sorted witness ─────────────────────────────────────────────────────── *)
(* A sorted array: only constructible via sort() — module hides the constructor. *)

module Sorted : sig
  type 'a t
  val sort          : 'a array -> 'a t  (* sorting IS the evidence *)
  val merge         : 'a t -> 'a t -> 'a t  (* merge of two sorted = sorted *)
  val binary_search : 'a t -> 'a -> bool
  val as_array      : 'a t -> 'a array
  val length        : 'a t -> int
  val is_empty      : 'a t -> bool
end = struct
  type 'a t = 'a array

  let sort arr =
    let a = Array.copy arr in
    Array.sort compare a;
    a

  (* Merge two sorted arrays — O(n+m); result is sorted by construction *)
  let merge a b =
    let na = Array.length a and nb = Array.length b in
    let result = Array.make (na + nb) a.(0) in (* placeholder init *)
    let i = ref 0 and j = ref 0 and k = ref 0 in
    while !i < na && !j < nb do
      if a.(!i) <= b.(!j) then begin
        result.(!k) <- a.(!i); incr i
      end else begin
        result.(!k) <- b.(!j); incr j
      end;
      incr k
    done;
    while !i < na do result.(!k) <- a.(!i); incr i; incr k done;
    while !j < nb do result.(!k) <- b.(!j); incr j; incr k done;
    result

  (* Binary search — safe because we know the array is sorted *)
  let binary_search arr target =
    let lo = ref 0 and hi = ref (Array.length arr - 1) in
    let found = ref false in
    while !lo <= !hi && not !found do
      let mid = (!lo + !hi) / 2 in
      let cmp = compare arr.(mid) target in
      if cmp = 0 then found := true
      else if cmp < 0 then lo := mid + 1
      else hi := mid - 1
    done;
    !found

  let as_array a = a
  let length = Array.length
  let is_empty a = Array.length a = 0
end

(* ── NonZeroInt witness ──────────────────────────────────────────────────── *)

module NonZeroInt : sig
  type t
  val create   : int -> t option
  val get      : t -> int
  val divide   : t -> int -> int  (* division never raises Division_by_zero *)
end = struct
  type t = int
  let create n = if n = 0 then None else Some n
  let get n = n
  let divide n dividend = dividend / n  (* safe: n ≠ 0 by construction *)
end

(* ── Authenticated witness ──────────────────────────────────────────────── *)

type unauthenticated = private Unauth_
type authenticated   = private Auth_

type 'auth session = {
  user_id : int;
}

module Session : sig
  val new_session   : unit -> unauthenticated session
  val authenticate  : unauthenticated session -> int -> string -> authenticated session
  val user_id       : authenticated session -> int
  val access_profile: authenticated session -> string
end = struct
  let new_session () = { user_id = 0 }

  (* authenticate: the act of checking credentials produces an authenticated witness *)
  let authenticate _ user_id _password =
    (* In reality: verify password hash *)
    { user_id }

  let user_id s = s.user_id

  let access_profile s =
    Printf.sprintf "Profile for user %d" s.user_id
end

let () =
  (* sorted witness is sorted *)
  let s = Sorted.sort [| 3; 1; 4; 1; 5; 9 |] in
  let v = Sorted.as_array s in
  Array.iter2 (fun a b -> assert (a <= b))
    (Array.sub v 0 (Array.length v - 1))
    (Array.sub v 1 (Array.length v - 1));
  print_endline "sorted witness: ok";

  (* binary search *)
  let s2 = Sorted.sort [| 1; 2; 3; 4; 5 |] in
  assert (Sorted.binary_search s2 3);
  let s3 = Sorted.sort [| 1; 2; 4; 5 |] in
  assert (not (Sorted.binary_search s3 3));
  print_endline "binary_search: ok";

  (* merge preserves sorted invariant *)
  let a = Sorted.sort [| 1; 3; 5 |] in
  let b = Sorted.sort [| 2; 4; 6 |] in
  let m = Sorted.merge a b in
  assert (Sorted.as_array m = [| 1; 2; 3; 4; 5; 6 |]);
  print_endline "merge: ok";

  (* non-zero rejects zero *)
  assert (NonZeroInt.create 0 = None);
  assert (NonZeroInt.create 1 <> None);
  let d = Option.get (NonZeroInt.create 7) in
  assert (NonZeroInt.divide d 49 = 7);
  print_endline "non_zero: ok";

  (* session witness *)
  let unauth = Session.new_session () in
  let auth   = Session.authenticate unauth 42 "secret" in
  assert (Session.user_id auth = 42);
  assert (Session.access_profile auth = "Profile for user 42");
  print_endline "session witness: ok";

  print_endline "All assertions passed."
