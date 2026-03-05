(* Structs with references in OCaml — GC manages lifetimes *)
type 'a cache = {
  key: string;
  value: 'a;
  description: string;
}

type document = {
  title: string;
  content: string;
}

type highlight = {
  document: document;
  start_line: int;
  end_line: int;
}

let make_highlight doc sl el = { document = doc; start_line = sl; end_line = el }

let show_highlight h =
  let lines = String.split_on_char '\n' h.document.content in
  let selected = List.filteri (fun i _ -> i >= h.start_line && i < h.end_line) lines in
  Printf.printf "Highlight [%d-%d] from %s:\n" h.start_line h.end_line h.document.title;
  List.iter (fun l -> Printf.printf "  | %s\n" l) selected

let () =
  let doc = {
    title = "My Document";
    content = "Line zero\nLine one\nLine two\nLine three";
  } in
  let h = make_highlight doc 1 3 in
  show_highlight h
