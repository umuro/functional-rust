(* Say (Numbers to English) *)
(* Recursive number-to-words conversion with large number support *)

let rec in_english_impl = function
  | 0L -> "zero" | 1L -> "one" | 2L -> "two" | 3L -> "three"
  | 4L -> "four" | 5L -> "five" | 6L -> "six" | 7L -> "seven"
  | 8L -> "eight" | 9L -> "nine" | 10L -> "ten" | 11L -> "eleven"
  | 12L -> "twelve" | 13L -> "thirteen" | 14L -> "fourteen"
  | 15L -> "fifteen" | 16L -> "sixteen" | 17L -> "seventeen"
  | 18L -> "eighteen" | 19L -> "nineteen" | 20L -> "twenty"
  | 30L -> "thirty" | 40L -> "forty" | 50L -> "fifty"
  | 60L -> "sixty" | 70L -> "seventy" | 80L -> "eighty" | 90L -> "ninety"
  | n when n <= 99L ->
    in_english_impl (Int64.mul 10L (Int64.div n 10L)) ^ "-" ^
    in_english_impl (Int64.rem n 10L)
  | n when n <= 999L ->
    in_english_impl (Int64.div n 100L) ^ " hundred" ^
    (let r = Int64.rem n 100L in if r = 0L then "" else " " ^ in_english_impl r)
  | n when n <= 999_999L ->
    in_english_impl (Int64.div n 1_000L) ^ " thousand" ^
    (let r = Int64.rem n 1_000L in if r = 0L then "" else " " ^ in_english_impl r)
  | n when n <= 999_999_999L ->
    in_english_impl (Int64.div n 1_000_000L) ^ " million" ^
    (let r = Int64.rem n 1_000_000L in if r = 0L then "" else " " ^ in_english_impl r)
  | n ->
    in_english_impl (Int64.div n 1_000_000_000L) ^ " billion" ^
    (let r = Int64.rem n 1_000_000_000L in if r = 0L then "" else " " ^ in_english_impl r)

let in_english n =
  if n < 0L || n >= 1_000_000_000_000L then Error "input out of range"
  else Ok (in_english_impl n)
