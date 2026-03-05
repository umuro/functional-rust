(* Zygomorphism in OCaml *)
(* Compute average and variance in one pass *)

let zygo f g xs =
  (* f is the "helper" algebra, g uses f's intermediate results *)
  List.fold_left (fun (acc_f, acc_g) x ->
    let new_f = f acc_f x in
    let new_g = g acc_g acc_f x in
    (new_f, new_g)
  ) (f [] (List.hd xs), g [] [] (List.hd xs))
  (List.tl xs)

(* Mean and sum of squares in one pass *)
let mean_and_ssq xs =
  let n = float_of_int (List.length xs) in
  let (sum, ssq) = List.fold_left (fun (s,q) x -> (s+.x, q+.x*.x)) (0.,0.) xs in
  (sum /. n, ssq /. n -. (sum/.n) *. (sum/.n))

let () =
  let xs = [2.;4.;4.;4.;5.;5.;7.;9.] in
  let (mean, variance) = mean_and_ssq xs in
  Printf.printf "mean=%.2f variance=%.2f stddev=%.2f\n" mean variance (sqrt variance)
