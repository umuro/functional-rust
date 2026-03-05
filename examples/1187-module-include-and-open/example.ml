(* Module Include and Open *)
(* Extend modules with include and local open *)

module ExtList = struct
  include List

  let sum = fold_left ( + ) 0
  let product = fold_left ( * ) 1

  let take n lst =
    let rec aux n acc = function
      | [] -> List.rev acc
      | _ when n <= 0 -> List.rev acc
      | x :: xs -> aux (n-1) (x :: acc) xs
    in aux n [] lst

  let drop n lst =
    let rec aux n = function
      | [] -> []
      | _ :: xs as l -> if n <= 0 then l else aux (n-1) xs
    in aux n lst
end

let () =
  let data = [1;2;3;4;5;6;7;8;9;10] in
  Printf.printf "Sum: %d\n" (ExtList.sum data);
  Printf.printf "First 3: %s\n"
    (String.concat " " (List.map string_of_int (ExtList.take 3 data)))
