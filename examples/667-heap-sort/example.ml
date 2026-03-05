(* Heap Sort in OCaml using Array *)

let heap_sort arr =
  let n = Array.length arr in
  let swap i j = let t = arr.(i) in arr.(i) <- arr.(j); arr.(j) <- t in
  let rec heapify size i =
    let largest = ref i in
    let left, right = 2*i+1, 2*i+2 in
    if left < size && arr.(left) > arr.(!largest) then largest := left;
    if right < size && arr.(right) > arr.(!largest) then largest := right;
    if !largest <> i then (swap i !largest; heapify size !largest)
  in
  for i = n/2-1 downto 0 do heapify n i done;
  for i = n-1 downto 1 do swap 0 i; heapify i 0 done

let () =
  let arr = [|12; 11; 13; 5; 6; 7|] in
  heap_sort arr;
  Printf.printf "Sorted: [|%s|]\n"
    (String.concat "; " (Array.to_list (Array.map string_of_int arr)))
