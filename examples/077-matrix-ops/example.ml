(* Matrix Operations — Functional 2D *)

(* Version 1: Transpose using List.init and List.nth *)
let transpose matrix =
  match matrix with
  | [] -> []
  | _ -> List.init (List.length (List.hd matrix)) (fun i ->
    List.map (fun row -> List.nth row i) matrix)

(* Version 2: Dot product and matrix multiply *)
let dot a b = List.fold_left2 (fun acc x y -> acc + x * y) 0 a b

let multiply a b =
  let bt = transpose b in
  List.map (fun row -> List.map (dot row) bt) a

let () =
  let a = [[1;2;3];[4;5;6]] in
  let b = [[7;8];[9;10];[11;12]] in
  let r = multiply a b in
  assert (r = [[58;64];[139;154]])
