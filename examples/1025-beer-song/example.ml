(* Beer Song *)
(* String generation with conditional formatting *)

let bottles = function
  | 0 -> "no more bottles"
  | 1 -> "1 bottle"
  | n -> string_of_int n ^ " bottles"

let verse = function
  | 0 -> String.capitalize_ascii (bottles 0) ^
    " of beer on the wall, " ^ bottles 0 ^ " of beer.\n" ^
    "Go to the store and buy some more, 99 bottles of beer on the wall."
  | n ->
    bottles n ^ " of beer on the wall, " ^ bottles n ^ " of beer.\n" ^
    "Take " ^ (if n > 1 then "one" else "it") ^ " down and pass it around, " ^
    bottles (n - 1) ^ " of beer on the wall."

let recite start count =
  List.init count (fun i -> verse (start - i))
  |> String.concat "\n\n"
