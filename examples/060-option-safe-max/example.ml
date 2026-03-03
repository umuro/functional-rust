(* Option Type — Safe List Maximum *)

(* Implementation 1: Recursive with pattern matching *)
let rec list_max = function
  | []     -> None
  | h :: t ->
    begin match list_max t with
    | None   -> Some h
    | Some m -> Some (max h m)
    end

(* Implementation 2: Using fold *)
let list_max_fold = function
  | []     -> None
  | h :: t -> Some (List.fold_left max h t)

(* Safe head *)
let safe_head = function
  | []    -> None
  | h :: _ -> Some h

(* Manual option_map *)
let option_map f = function
  | None   -> None
  | Some x -> Some (f x)

(* Tests *)
let () =
  let nums = [3; 1; 4; 1; 5; 9; 2; 6] in
  assert (list_max nums = Some 9);
  assert (list_max_fold nums = Some 9);
  assert (list_max [] = None);
  assert (list_max_fold [] = None);
  assert (list_max [42] = Some 42);
  assert (list_max [-5; -1; -10] = Some (-1));
  assert (safe_head [1; 2; 3] = Some 1);
  assert (safe_head [] = None);
  assert (option_map (fun x -> x * 2) (Some 5) = Some 10);
  assert (option_map (fun x -> x * 2) None = None);
  assert (option_map (fun x -> x * 2) (list_max nums) = Some 18);
  Printf.printf "All option-safe-max tests passed!\n"
