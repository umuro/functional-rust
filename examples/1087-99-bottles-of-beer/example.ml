(* 99 bottles of beer *)
(* Rosetta Code 99 bottles of beer implementation in OCaml *)

for n = 99 downto 1 do
  Printf.printf "%d bottles of beer on the wall\n" n;
  Printf.printf "%d bottles of beer\n" n;
  Printf.printf "Take one down, pass it around\n";
  Printf.printf "%d bottles of beer on the wall\n\n" (pred n);
done
