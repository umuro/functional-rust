(* Type Classes via Modules — Show and Eq *)
(* Simulate type classes with module signatures *)

module type SHOW = sig
  type t
  val show : t -> string
end

module type EQ = sig
  type t
  val equal : t -> t -> bool
end

let print_list (type a) (module S : SHOW with type t = a) (lst : a list) =
  Printf.printf "[%s]\n" (String.concat "; " (List.map S.show lst))

let dedup (type a) (module E : EQ with type t = a) (lst : a list) =
  List.fold_left (fun acc x ->
    if List.exists (E.equal x) acc then acc else x :: acc
  ) [] lst |> List.rev

let () =
  let module IntShow = struct type t = int let show = string_of_int end in
  let module IntEq = struct type t = int let equal = Int.equal end in
  print_list (module IntShow) [1;2;3;4;5];
  let d = dedup (module IntEq) [1;2;1;3;2;4;3;5] in
  print_list (module IntShow) d
