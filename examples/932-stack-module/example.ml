(* 932: Stack Module with Signature

   OCaml uses module types (signatures) to enforce abstraction.
   We define a STACK module type, then provide two implementations:
   a persistent (immutable) list-backed stack and a mutable one. *)

(* ── Module type (signature) ─────────────────────────────────────────────── *)

module type STACK = sig
  type 'a t
  val empty   : 'a t
  val is_empty : 'a t -> bool
  val push    : 'a -> 'a t -> 'a t
  val peek    : 'a t -> 'a option
  val pop     : 'a t -> ('a * 'a t) option  (* returns (top, rest) *)
  val size    : 'a t -> int
end

(* ── Persistent (immutable) stack — backed by a list ─────────────────────── *)

module ListStack : STACK = struct
  type 'a t = 'a list

  let empty = []
  let is_empty = function [] -> true | _ -> false
  let push x s = x :: s
  let peek = function [] -> None | x :: _ -> Some x
  let pop  = function [] -> None | x :: rest -> Some (x, rest)
  let size = List.length
end

(* ── Mutable stack — backed by a ref to a list ───────────────────────────── *)

module MutStack = struct
  type 'a t = { mutable items : 'a list }

  let create () = { items = [] }
  let is_empty s = s.items = []
  let push s x   = s.items <- x :: s.items
  let peek s     = match s.items with [] -> None | x :: _ -> Some x
  let pop s      = match s.items with
    | [] -> None
    | x :: rest -> s.items <- rest; Some x
  let size s = List.length s.items
end

(* ── Using the signature as a functor parameter ──────────────────────────── *)

(* A functor that works with any STACK implementation *)
module StackUtils (S : STACK) = struct
  (* Push a list of items onto the stack (left to right) *)
  let push_all items s =
    List.fold_left (fun acc x -> S.push x acc) s items

  (* Convert stack to list (top-first) *)
  let to_list s =
    let rec go acc s =
      match S.pop s with
      | None -> List.rev acc
      | Some (x, rest) -> go (x :: acc) rest
    in
    go [] s
end

module LSU = StackUtils(ListStack)

let () =
  (* Persistent stack *)
  let s0 = ListStack.empty in
  let s1 = ListStack.push 1 s0 in
  let s2 = ListStack.push 2 s1 in
  let s3 = ListStack.push 3 s2 in
  assert (ListStack.size s3 = 3);
  assert (ListStack.peek s3 = Some 3);

  (match ListStack.pop s3 with
   | Some (top, rest) ->
     assert (top = 3);
     assert (ListStack.peek rest = Some 2);
     (* Original s3 unchanged — persistent *)
     assert (ListStack.peek s3 = Some 3)
   | None -> failwith "expected Some");

  assert (ListStack.is_empty ListStack.empty);
  assert (ListStack.peek ListStack.empty = None);
  assert (ListStack.pop ListStack.empty = None);

  (* Single element *)
  let s = ListStack.push 42 ListStack.empty in
  assert (ListStack.size s = 1);
  assert (ListStack.peek s = Some 42);
  (match ListStack.pop s with
   | Some (_, rest) -> assert (ListStack.is_empty rest)
   | None -> failwith "expected Some");

  (* Mutable stack *)
  let ms = MutStack.create () in
  assert (MutStack.is_empty ms);
  MutStack.push ms 1;
  MutStack.push ms 2;
  MutStack.push ms 3;
  assert (MutStack.size ms = 3);
  assert (MutStack.pop ms = Some 3);
  assert (MutStack.peek ms = Some 2);

  (* StackUtils functor *)
  let s4 = LSU.push_all [1; 2; 3] ListStack.empty in
  assert (LSU.to_list s4 = [3; 2; 1]);  (* LIFO order *)

  print_endline "932-stack-module: all tests passed"
