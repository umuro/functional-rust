type category = Ones | Twos | Threes | Fours | Fives | Sixes
  | FullHouse | FourOfAKind | LittleStraight | BigStraight | Yacht | Choice

let count dice n = List.length (List.filter ((=) n) dice)

let score dice = function
  | Ones -> count dice 1 | Twos -> 2 * count dice 2
  | Threes -> 3 * count dice 3 | Fours -> 4 * count dice 4
  | Fives -> 5 * count dice 5 | Sixes -> 6 * count dice 6
  | Choice -> List.fold_left (+) 0 dice
  | Yacht -> if List.for_all ((=) (List.hd dice)) dice then 50 else 0
  | FullHouse ->
    let sorted = List.sort compare dice in
    (match sorted with
     | [a;b;c;d;e] when a=b && b=c && d=e && c<>d -> List.fold_left (+) 0 dice
     | [a;b;c;d;e] when a=b && c=d && d=e && b<>c -> List.fold_left (+) 0 dice
     | _ -> 0)
  | FourOfAKind ->
    (try
       let v = List.find (fun n -> count dice n >= 4) (List.sort_uniq compare dice) in
       4 * v
     with Not_found -> 0)
  | LittleStraight ->
    if List.sort compare dice = [1;2;3;4;5] then 30 else 0
  | BigStraight ->
    if List.sort compare dice = [2;3;4;5;6] then 30 else 0

let () =
  assert (score [5;5;5;5;5] Yacht = 50);
  assert (score [2;2;3;3;3] FullHouse = 13);
  assert (score [1;2;3;4;5] Choice = 15);
  assert (score [1;2;3;4;5] LittleStraight = 30);
  assert (score [2;3;4;5;6] BigStraight = 30);
  print_endline "ok"
