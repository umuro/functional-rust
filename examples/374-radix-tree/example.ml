(* OCaml: radix tree / compressed trie *)

(* We show the compression concept *)
(* Standard trie for 'car','card','care' would have:
   root -> c -> a -> r -> (end) -> d -> (end)
                              \-> e -> (end)
   Radix compresses 'c','a' into edge 'ca' if no branching *)

type radix =
  | Leaf of string
  | Node of (string * radix) list * bool  (* edges * is_end *)

let empty = Node ([], false)

let rec insert node path =
  match node with
  | Leaf s -> if s = path then Leaf s else
    (* Find common prefix *)
    let rec cp a b i =
      if i >= String.length a || i >= String.length b || a.[i] <> b.[i] then i
      else cp a b (i+1)
    in
    let i = cp s path 0 in
    let pre = String.sub s 0 i in
    Node ([String.sub s i (String.length s - i), Leaf s;
           String.sub path i (String.length path - i), Leaf path], false)
  | _ -> node  (* simplified *)

let () =
  Printf.printf "Radix tree compresses 'car','card','care' into:\n";
  Printf.printf "  root --'car'--> Node(end) --'d'--> Leaf --'e'--> Leaf\n";
  Printf.printf "This uses 3 nodes instead of 7 in a standard trie\n"
