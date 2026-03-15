(* 456. Lazy init – OCaml *)
let config = lazy (
  Printf.printf "init config\n%!";
  [("host","localhost");("port","8080")])

let primes_under_50 = lazy (
  Printf.printf "computing primes\n%!";
  let s = Array.make 51 true in
  s.(0)<-false; s.(1)<-false;
  for i=2 to 7 do if s.(i) then
    let j=ref(i*i) in while !j<=50 do s.(!j)<-false; j:= !j+i done
  done;
  Array.to_list (Array.init 51 (fun i -> if s.(i) then [i] else []) |> Array.concat)
)

let () =
  let v1 = Lazy.force config in
  let v2 = Lazy.force config in  (* not recomputed *)
  assert (v1 == v2);             (* same object *)
  Printf.printf "host=%s\n" (List.assoc "host" v1);
  Printf.printf "primes: %s\n"
    (String.concat "," (List.map string_of_int (Lazy.force primes_under_50)))
