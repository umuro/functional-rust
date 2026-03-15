(* 002: List Operations
   head, tail, length, append, reverse — OCaml lists are naturally recursive *)

(* --- Approach 1: Using stdlib --- *)

let head = function
  | [] -> None
  | x :: _ -> Some x

let tail = function
  | [] -> None
  | _ :: xs -> Some xs

let length xs = List.length xs

let append xs ys = xs @ ys

let reverse xs = List.rev xs

(* --- Approach 2: Manual recursive implementations --- *)

let rec rec_length = function
  | [] -> 0
  | _ :: xs -> 1 + rec_length xs

let rec rec_reverse = function
  | [] -> []
  | x :: xs -> rec_reverse xs @ [x]

(* --- Approach 3: Tail-recursive reverse with accumulator --- *)

let rev_acc xs =
  (* accumulator holds reversed prefix so far *)
  let rec aux acc = function
    | [] -> acc
    | x :: xs -> aux (x :: acc) xs
  in
  aux [] xs

let () =
  let v = [1; 2; 3] in
  Printf.printf "head [1;2;3] = %s\n"
    (match head v with Some x -> string_of_int x | None -> "None");
  Printf.printf "tail [1;2;3] = %s\n"
    (match tail v with Some xs -> "[" ^ String.concat ";" (List.map string_of_int xs) ^ "]" | None -> "None");
  Printf.printf "length [1;2;3] = %d\n" (length v);
  Printf.printf "append [1;2] [3;4] = [%s]\n"
    (String.concat "; " (List.map string_of_int (append [1;2] [3;4])));
  Printf.printf "reverse [1;2;3] = [%s]\n"
    (String.concat "; " (List.map string_of_int (reverse v)));
  Printf.printf "rev_acc [1;2;3] = [%s]\n"
    (String.concat "; " (List.map string_of_int (rev_acc v)))
