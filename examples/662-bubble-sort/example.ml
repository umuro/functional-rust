(* Bubble Sort in OCaml *)

let bubble_sort arr =
  let a = Array.copy arr in
  let n = Array.length a in
  for i = 0 to n - 1 do
    for j = 0 to n - 2 - i do
      if a.(j) > a.(j + 1) then begin
        let tmp = a.(j) in
        a.(j) <- a.(j + 1);
        a.(j + 1) <- tmp
      end
    done
  done;
  a

let () =
  let arr = [|64; 34; 25; 12; 22; 11; 90|] in
  let sorted = bubble_sort arr in
  Printf.printf "Sorted: [|%s|]\n"
    (String.concat "; " (Array.to_list (Array.map string_of_int sorted)))
