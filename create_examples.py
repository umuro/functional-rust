import os
import re

base = '/home/node/hightechmind2024/functional-rust/examples'
# mapping new_number -> (original_number, title, code)
# code strings need to be exact
mapping = [
    (1181, '029', 'List.partition — Divide list by predicate',
     '''let numbers = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
let (small, big) = List.partition (fun x -> x <= 5) numbers
let () = Printf.printf "Small: %s\\n"
  (String.concat " " (List.map string_of_int small))
let () = Printf.printf "Big: %s\\n"
  (String.concat " " (List.map string_of_int big))'''),
    (1182, '030', 'List.flatten — Flatten Nested Lists',
     '''let nested = [[1; 2]; [3; 4; 5]; [6]; [7; 8; 9; 10]]
let flat = List.flatten nested
let () = Printf.printf "Flat: %s\\n"
  (String.concat " " (List.map string_of_int flat))
(* Also useful: List.concat_map *)
let pairs = List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]'''),
    (1183, '036', 'Array.blit — Copy Subarray',
     '''let src = [| 10; 20; 30; 40; 50 |]
let dst = Array.make 8 0
let () = Array.blit src 1 dst 2 3
(* dst is now [| 0; 0; 20; 30; 40; 0; 0; 0 |] *)
let () = Array.iter (fun x -> Printf.printf "%d " x) dst'''),
    (1184, '037', 'Array.make and Array.make_matrix — Multi-dimensional Arrays',
     '''let zeros = Array.make 5 0
let matrix = Array.make_matrix 3 4 0.0
let () = matrix.(1).(2) <- 42.0
let () =
  Array.iter (fun row ->
    Array.iter (fun x -> Printf.printf "%.0f " x) row;
    print_newline ()
  ) matrix'''),
    (1185, '038', 'String.split_on_char — Tokenize a String',
     '''let csv_line = "Alice,30,Engineer,Amsterdam"
let fields = String.split_on_char ',' csv_line
let () = List.iteri (fun i f -> Printf.printf "Field %d: %s\\n" i f) fields

let words = String.split_on_char ' ' "  hello   world  "
let nonempty = List.filter (fun s -> s <> "") words'''),
]

def slugify(title):
    # remove special chars, replace spaces with hyphens, lowercase
    s = re.sub(r'[^\w\s-]', '', title)
    s = re.sub(r'[-\s]+', '-', s).strip().lower()
    return s

for new_num, orig, title, code in mapping:
    slug = slugify(title)
    dirname = f'{new_num}-{slug}'
    path = os.path.join(base, dirname)
    os.makedirs(path, exist_ok=True)
    ml_path = os.path.join(path, 'example.ml')
    with open(ml_path, 'w') as f:
        f.write(code)
    print(f'Created {dirname}')
    # also write a minimal README.md with title
    readme_path = os.path.join(path, 'README.md')
    with open(readme_path, 'w') as f:
        f.write(f'# {title}\\n\\n')
        f.write(f'Original OCaml example {orig}.\\n')
        f.write('Converted to Rust using Functional Rust style.\\n')
    # optionally create COMPARISON.md placeholder
    comp_path = os.path.join(path, 'COMPARISON.md')
    with open(comp_path, 'w') as f:
        f.write('# Comparison\\n\\n')
        f.write('## OCaml\\n\\n```ocaml\\n')
        f.write(code)
        f.write('\\n```\\n\\n## Rust\\n\\n*TODO*\\n')
    print(f'  Wrote example.ml ({len(code)} chars)')

print('All directories created.')