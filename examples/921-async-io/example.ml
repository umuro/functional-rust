(* OCaml: file and network I/O patterns *)

let read_file_lines filename =
  let ic = open_in filename in
  let rec loop acc =
    match input_line ic with
    | line -> loop (line :: acc)
    | exception End_of_file -> close_in ic; List.rev acc
  in loop []

let count_words text =
  String.split_on_char ' ' text
  |> List.filter (fun s -> String.length s > 0)
  |> List.length

let process_text text =
  let lines = String.split_on_char '\n' text in
  let words = count_words text in
  let chars = String.length text in
  (List.length lines, words, chars)

let () =
  let text = "Hello world\nFoo bar baz\nOne two three four" in
  let (lines, words, chars) = process_text text in
  Printf.printf "Lines: %d, Words: %d, Chars: %d\n" lines words chars
