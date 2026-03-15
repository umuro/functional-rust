(* 477. String trimming – OCaml *)
let () =
  Printf.printf "'%s'\n" (String.trim "  hello  ");
  let trim_char c s =
    let n=String.length s in
    let lo=ref 0 and hi=ref(n-1) in
    while !lo <= !hi && s.[!lo]=c do incr lo done;
    while !hi >= !lo && s.[!hi]=c do decr hi done;
    if !lo > !hi then "" else String.sub s !lo (!hi - !lo + 1)
  in
  Printf.printf "'%s'\n" (trim_char '*' "***hello***");
  let lines=["  line1  ";"  line2  "] in
  List.iter (fun l -> Printf.printf "'%s'\n" (String.trim l)) lines
