(* 478. String searching – OCaml *)
let contains s sub =
  let ls=String.length s and lp=String.length sub in
  if lp=0 then true
  else let found=ref false in
       for i=0 to ls-lp do
         if String.sub s i lp = sub then found:=true done;
       !found

let () =
  let s = "Hello, World! Hello, OCaml!" in
  Printf.printf "contains World: %b\n" (contains s "World");
  Printf.printf "starts Hello: %b\n" (String.length s >= 5 && String.sub s 0 5 = "Hello");
  Printf.printf "ends !: %b\n" (s.[String.length s - 1] = '!');
  (match String.index_opt s ',' with
   | Some i -> Printf.printf "comma at %d\n" i | None->())
