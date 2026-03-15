(* 093: Windows and Chunks
   OCaml standard library lacks built-in windows/chunks; easy to implement *)

(* --- Approach 1: Sliding windows of size n --- *)

let windows n xs =
  (* Generate all contiguous sub-lists of length n *)
  let len = List.length xs in
  if n > len || n <= 0 then []
  else
    let arr = Array.of_list xs in
    List.init (len - n + 1) (fun i ->
      Array.to_list (Array.sub arr i n))

(* --- Approach 2: Non-overlapping chunks of size n --- *)

let chunks n xs =
  if n <= 0 then []
  else
    let rec aux acc = function
      | [] -> List.rev acc
      | xs ->
        let len = List.length xs in
        let chunk = List.filteri (fun i _ -> i < n) xs in
        let rest  = if len <= n then [] else List.filteri (fun i _ -> i >= n) xs in
        aux (chunk :: acc) rest
    in
    aux [] xs

(* More efficient chunk using Array.sub *)
let chunks_arr n xs =
  if n <= 0 then []
  else begin
    let arr = Array.of_list xs in
    let len = Array.length arr in
    let count = (len + n - 1) / n in   (* ceiling division *)
    List.init count (fun i ->
      let start = i * n in
      let size  = min n (len - start) in
      Array.to_list (Array.sub arr start size))
  end

let () =
  let v = [1;2;3;4;5] in
  Printf.printf "windows 3 [1..5] = %s\n"
    (String.concat ", "
      (List.map (fun w -> "[" ^ String.concat ";" (List.map string_of_int w) ^ "]")
        (windows 3 v)));

  Printf.printf "windows 2 [1;2;3] = %s\n"
    (String.concat ", "
      (List.map (fun w -> "[" ^ String.concat ";" (List.map string_of_int w) ^ "]")
        (windows 2 [1;2;3])));

  Printf.printf "chunks 2 [1..5] = %s\n"
    (String.concat ", "
      (List.map (fun c -> "[" ^ String.concat ";" (List.map string_of_int c) ^ "]")
        (chunks_arr 2 v)));

  Printf.printf "chunks 3 [1..6] = %s\n"
    (String.concat ", "
      (List.map (fun c -> "[" ^ String.concat ";" (List.map string_of_int c) ^ "]")
        (chunks_arr 3 [1;2;3;4;5;6])))
