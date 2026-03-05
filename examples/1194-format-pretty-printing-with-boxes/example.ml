(* Format — Pretty Printing with Boxes *)
(* Use Format module for structured output *)

let pp_list pp_item fmt lst =
  Format.fprintf fmt "[@[<hov 2>";
  List.iteri (fun i x ->
    if i > 0 then Format.fprintf fmt ";@ ";
    pp_item fmt x
  ) lst;
  Format.fprintf fmt "@]]"

let pp_int fmt n = Format.fprintf fmt "%d" n
let pp_string fmt s = Format.fprintf fmt "%S" s

let () =
  Format.printf "Numbers: %a@." (pp_list pp_int) [1;2;3;4;5;6;7;8;9;10];
  Format.printf "Words: %a@." (pp_list pp_string) ["hello"; "world"; "ocaml"]
