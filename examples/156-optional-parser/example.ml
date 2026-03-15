(* Example 156: Optional Parser *)
(* opt: make a parser optional, returns Option *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let tag expected : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else Error (Printf.sprintf "Expected \"%s\"" expected)

(* Approach 1: opt — wrap result in option, always succeeds *)
let opt (p : 'a parser) : 'a option parser = fun input ->
  match p input with
  | Ok (v, rest) -> Ok (Some v, rest)
  | Error _ -> Ok (None, input)

(* Approach 2: with_default — provide a fallback value *)
let with_default (default : 'a) (p : 'a parser) : 'a parser = fun input ->
  match p input with
  | Ok _ as result -> result
  | Error _ -> Ok (default, input)

(* Approach 3: peek — check if parser would succeed without consuming *)
let peek (p : 'a parser) : 'a option parser = fun input ->
  match p input with
  | Ok (v, _) -> Ok (Some v, input)  (* don't advance! *)
  | Error _ -> Ok (None, input)

(* Tests *)
let () =
  let digit = satisfy (fun c -> c >= '0' && c <= '9') "digit" in

  (* opt: success case *)
  assert (opt (tag "+") "+42" = Ok (Some "+", "42"));
  (* opt: failure returns None without consuming *)
  assert (opt (tag "+") "42" = Ok (None, "42"));

  (* with_default *)
  assert (with_default '+' (satisfy (fun c -> c = '+' || c = '-') "sign") "+5" = Ok ('+', "5"));
  assert (with_default '+' (satisfy (fun c -> c = '+' || c = '-') "sign") "5" = Ok ('+', "5"));

  (* peek *)
  (match peek digit "123" with
   | Ok (Some '1', "123") -> ()  (* input not consumed *)
   | _ -> failwith "Peek test failed");

  (match peek digit "abc" with
   | Ok (None, "abc") -> ()
   | _ -> failwith "Peek test 2 failed");

  print_endline "✓ All tests passed"
