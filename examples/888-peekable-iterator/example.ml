(* Example 094: Peekable Iterator *)
(* Lookahead parsing *)

(* Approach 1: Manual peekable with buffer *)
type 'a peekable = {
  mutable peeked : 'a option;
  next_fn : unit -> 'a option;
}

let peekable_of_list lst =
  let rest = ref lst in
  { peeked = None;
    next_fn = fun () ->
      match !rest with
      | [] -> None
      | x :: xs -> rest := xs; Some x
  }

let peek p =
  match p.peeked with
  | Some _ as v -> v
  | None ->
    let v = p.next_fn () in
    p.peeked <- v;
    v

let next p =
  match p.peeked with
  | Some _ as v -> p.peeked <- None; v
  | None -> p.next_fn ()

(* Approach 2: Simple tokenizer using peek *)
type token = Num of int | Plus | Minus | Star | Lparen | Rparen

let tokenize s =
  let chars = peekable_of_list (List.init (String.length s) (String.get s)) in
  let tokens = ref [] in
  let rec aux () =
    match peek chars with
    | None -> ()
    | Some ' ' -> ignore (next chars); aux ()
    | Some c when c >= '0' && c <= '9' ->
      let buf = Buffer.create 4 in
      let rec read_num () =
        match peek chars with
        | Some c when c >= '0' && c <= '9' ->
          Buffer.add_char buf c; ignore (next chars); read_num ()
        | _ -> ()
      in
      read_num ();
      tokens := Num (int_of_string (Buffer.contents buf)) :: !tokens;
      aux ()
    | Some '+' -> ignore (next chars); tokens := Plus :: !tokens; aux ()
    | Some '-' -> ignore (next chars); tokens := Minus :: !tokens; aux ()
    | Some '*' -> ignore (next chars); tokens := Star :: !tokens; aux ()
    | Some '(' -> ignore (next chars); tokens := Lparen :: !tokens; aux ()
    | Some ')' -> ignore (next chars); tokens := Rparen :: !tokens; aux ()
    | Some _ -> ignore (next chars); aux ()
  in
  aux ();
  List.rev !tokens

(* Approach 3: Take while with peek *)
let take_while_peek pred p =
  let rec aux acc =
    match peek p with
    | Some x when pred x -> ignore (next p); aux (x :: acc)
    | _ -> List.rev acc
  in
  aux []

let skip_while_peek pred p =
  let rec aux () =
    match peek p with
    | Some x when pred x -> ignore (next p); aux ()
    | _ -> ()
  in
  aux ()

(* Tests *)
let () =
  let p = peekable_of_list [1; 2; 3] in
  assert (peek p = Some 1);
  assert (peek p = Some 1);
  assert (next p = Some 1);
  assert (next p = Some 2);
  assert (peek p = Some 3);
  assert (next p = Some 3);
  assert (next p = None);

  let tokens = tokenize "12 + 34 * 5" in
  assert (List.length tokens = 5);
  assert (List.hd tokens = Num 12);

  let p2 = peekable_of_list [1; 2; 3; 10; 20; 30] in
  let small = take_while_peek (fun x -> x < 10) p2 in
  assert (small = [1; 2; 3]);
  assert (peek p2 = Some 10);

  Printf.printf "✓ All tests passed\n"
