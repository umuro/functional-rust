(* OCaml: Union-Find with arrays *)

let parent = Array.init 10 (fun i -> i)
let rank   = Array.make 10 0

let rec find x =
  if parent.(x) = x then x
  else begin
    parent.(x) <- find parent.(x);  (* path compression *)
    parent.(x)
  end

let union x y =
  let rx = find x and ry = find y in
  if rx = ry then ()
  else if rank.(rx) < rank.(ry) then parent.(rx) <- ry
  else if rank.(rx) > rank.(ry) then parent.(ry) <- rx
  else begin parent.(ry) <- rx; rank.(rx) <- rank.(rx)+1 end

let () =
  union 0 1; union 2 3; union 0 2;
  Printf.printf "Connected(0,3): %b\n" (find 0 = find 3);
  Printf.printf "Connected(0,4): %b\n" (find 0 = find 4)
