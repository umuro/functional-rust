(* Example 151: Introduction to Parser Combinators *)
(* OCaml approach using Angstrom-style concepts *)

(* Approach 1: Basic parser type as a function *)
type 'a parse_result = Ok of 'a * string | Error of string

type 'a parser = string -> 'a parse_result

let run_parser (p : 'a parser) (input : string) : 'a parse_result =
  p input

(* A parser that consumes a single character 'a' *)
let parse_a : char parser = fun input ->
  if String.length input > 0 && input.[0] = 'a' then
    Ok ('a', String.sub input 1 (String.length input - 1))
  else
    Error (Printf.sprintf "Expected 'a', got '%s'" input)

(* Approach 2: Using a module to encapsulate parser operations *)
module Parser = struct
  type 'a t = string -> 'a parse_result

  let return (x : 'a) : 'a t = fun input -> Ok (x, input)

  let fail (msg : string) : 'a t = fun _input -> Error msg

  let run (p : 'a t) (input : string) = p input

  (* Parse a specific character *)
  let char (c : char) : char t = fun input ->
    if String.length input > 0 && input.[0] = c then
      Ok (c, String.sub input 1 (String.length input - 1))
    else
      Error (Printf.sprintf "Expected '%c'" c)
end

(* Approach 3: Using Angstrom-like combinators *)
(* In real OCaml, you'd use: open Angstrom *)
(* let p = char 'a' *)
(* let result = parse_string ~consume:All p "a" *)

(* Simulating Angstrom's interface *)
module Angstrom_like = struct
  type 'a t = string -> ('a * string, string) result

  let char c : char t = fun input ->
    if String.length input > 0 && input.[0] = c then
      Result.ok (c, String.sub input 1 (String.length input - 1))
    else
      Result.error (Printf.sprintf "Expected '%c'" c)

  let parse_string p input =
    match p input with
    | Result.Ok (v, rest) ->
      if String.length rest = 0 then Result.Ok v
      else Result.Error "Unconsumed input"
    | Result.Error e -> Result.Error e
end

(* Tests *)
let () =
  (* Test Approach 1 *)
  (match run_parser parse_a "abc" with
   | Ok ('a', "bc") -> ()
   | _ -> failwith "Test 1 failed");

  (match run_parser parse_a "xyz" with
   | Error _ -> ()
   | _ -> failwith "Test 2 failed");

  (* Test Approach 2 *)
  (match Parser.run (Parser.char 'b') "bcd" with
   | Ok ('b', "cd") -> ()
   | _ -> failwith "Test 3 failed");

  (match Parser.run (Parser.return 42) "hello" with
   | Ok (42, "hello") -> ()
   | _ -> failwith "Test 4 failed");

  (* Test Approach 3 *)
  (match Angstrom_like.parse_string (Angstrom_like.char 'x') "x" with
   | Result.Ok 'x' -> ()
   | _ -> failwith "Test 5 failed");

  print_endline "✓ All tests passed"
