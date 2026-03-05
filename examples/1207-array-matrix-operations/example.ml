(* Array — Matrix Operations *)
(* Basic matrix math with arrays *)

let mat_mul a b =
  let rows = Array.length a and cols = Array.length b.(0) in
  let k = Array.length b in
  Array.init rows (fun i ->
    Array.init cols (fun j ->
      let sum = ref 0 in
      for p = 0 to k - 1 do
        sum := !sum + a.(i).(p) * b.(p).(j)
      done;
      !sum
    )
  )

let print_mat m =
  Array.iter (fun row ->
    Array.iter (fun x -> Printf.printf "%3d " x) row;
    print_newline ()
  ) m

let a = [| [|1;2|]; [|3;4|] |]
let b = [| [|5;6|]; [|7;8|] |]
let c = mat_mul a b
let () = print_mat c
