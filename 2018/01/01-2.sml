fun readInput filename =
    let val file = TextIO.openIn filename
        val changes = TextIO.inputAll file
        val _ = TextIO.closeIn file
    in String.tokens (fn c => c = #"\n") changes
    end;

fun intFromString s =
    case Int.fromString s of
         SOME i => i
       | NONE => raise Fail ("String '" ^ s ^ "' not an Int.");

fun nextFrequency (index, changes, frequency) =
    let val change = List.nth (changes, index)
    in change + frequency
    end;

fun firstRepeat (index, changes, current, frequencies) =
    let val frequency = nextFrequency (index, changes, current)
    in case List.exists (fn y => frequency = y) frequencies of
        true => frequency
    |   false => case index = List.length (changes) - 1 of
            true => firstRepeat (0, changes, frequency, frequency::frequencies)
        |   false => firstRepeat (index + 1, changes, frequency, frequency::frequencies)
    end;

val strings = readInput "input.txt";
val changes = map intFromString strings;
val repeated = firstRepeat (0, changes, 0, []);
