(* Returns None for empty list instead of raising an exception *)
let rec list_max = function
  | []     -> None
  | h :: t ->
    begin match list_max t with
    | None   -> Some h
    | Some m -> Some (max h m)
    end

(* Safe head — another classic option use *)
let safe_head = function
  | []    -> None
  | h :: _ -> Some h

(* Chaining options with map *)
let option_map f = function
  | None   -> None
  | Some x -> Some (f x)

let () =
  let nums = [3; 1; 4; 1; 5; 9; 2; 6] in
  (match list_max nums with
   | None   -> print_endline "empty list"
   | Some m -> Printf.printf "max = %d\n" m);
  (match list_max [] with
   | None   -> print_endline "max of [] = None"
   | Some m -> Printf.printf "max = %d\n" m);
  (* Double the maximum if it exists *)
  let doubled_max = option_map (fun x -> x * 2) (list_max nums) in
  match doubled_max with
  | None   -> ()
  | Some v -> Printf.printf "doubled max = %d\n" v
