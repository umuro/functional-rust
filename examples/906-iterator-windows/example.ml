(* 262. Sliding windows over slices - OCaml *)

let windows n arr =
  let len = Array.length arr in
  if n > len then [||]
  else Array.init (len - n + 1) (fun i -> Array.sub arr i n)

let () =
  let arr = [|1; 2; 3; 4; 5|] in
  let ws = windows 3 arr in
  Printf.printf "Windows of 3:\n";
  Array.iter (fun w ->
    Printf.printf "[%s]\n"
      (String.concat "; " (Array.to_list (Array.map string_of_int w)))
  ) ws;

  let k = 3 in
  let avgs = Array.map (fun window ->
    let sum = Array.fold_left (+) 0 window in
    float_of_int sum /. float_of_int k
  ) (windows k arr) in
  Printf.printf "Moving averages: %s\n"
    (String.concat ", "
      (Array.to_list (Array.map (Printf.sprintf "%.1f") avgs)));

  let pairs = windows 2 arr in
  let increasing = Array.for_all (fun w -> w.(0) < w.(1)) pairs in
  Printf.printf "Monotonically increasing: %b\n" increasing
