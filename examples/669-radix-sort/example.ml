(* Radix Sort in OCaml - LSD *)

let counting_sort_by_digit arr exp =
  let n = Array.length arr in
  let output = Array.make n 0 in
  let count = Array.make 10 0 in
  Array.iter (fun x -> 
    let d = (x / exp) mod 10 in
    count.(d) <- count.(d) + 1) arr;
  for i = 1 to 9 do count.(i) <- count.(i) + count.(i-1) done;
  for i = n - 1 downto 0 do
    let d = (arr.(i) / exp) mod 10 in
    count.(d) <- count.(d) - 1;
    output.(count.(d)) <- arr.(i)
  done;
  Array.blit output 0 arr 0 n

let radix_sort arr =
  if Array.length arr = 0 then () else
  let max_val = Array.fold_left max arr.(0) arr in
  let rec loop exp =
    if max_val / exp > 0 then (counting_sort_by_digit arr exp; loop (exp * 10))
  in loop 1

let () =
  let arr = [|170; 45; 75; 90; 802; 24; 2; 66|] in
  radix_sort arr;
  Printf.printf "Sorted: [|%s|]\n"
    (String.concat "; " (Array.to_list (Array.map string_of_int arr)))
