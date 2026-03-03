(* Natural transformations: morphisms in the functor category.
   Key property: commutes with fmap — "structure preserving" *)

(* Safe head: list -> option (natural transformation) *)
let safe_head lst = match lst with [] -> None | x :: _ -> Some x

(* Safe last *)
let safe_last lst = match List.rev lst with [] -> None | x :: _ -> Some x

(* Horizontal composition: nat trans can be composed *)
let reverse_opt : 'a option -> 'a option = fun x -> x  (* identity on option *)

(* Naturality square verification *)
let verify_naturality f nat lst =
  let lhs = nat (List.map f lst) in  (* map then transform *)
  let rhs = Option.map f (nat lst) in  (* transform then map *)
  lhs = rhs

let () =
  (* Verify safe_head is natural w.r.t. string_of_int *)
  let lists = [[] ; [1]; [1;2;3]; [42;0;-1]] in
  let natural = List.for_all (verify_naturality string_of_int safe_head) lists in
  Printf.printf "safe_head natural? %b\n" natural;

  (* Composition of natural transformations *)
  (* list -[safe_head]-> option -[option_to_list]-> list *)
  let option_to_list o = match o with None -> [] | Some x -> [x] in
  let nat_composed lst = option_to_list (safe_head lst) in

  Printf.printf "composed [1;2;3]: [%s]\n"
    (nat_composed [1;2;3] |> List.map string_of_int |> String.concat ";");
  Printf.printf "composed []: [%s]\n"
    (nat_composed [] |> List.map string_of_int |> String.concat ";")
