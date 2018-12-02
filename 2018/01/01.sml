fun readInput(filename) =
    let val file = TextIO.openIn filename
        val frequencies = TextIO.inputAll file
        val _ = TextIO.closeIn file
    in String.tokens (fn c => c = #"\n") frequencies
    end

fun intFromString s =
    case Int.fromString s of
         SOME i => i
       | NONE => raise Fail ("Could not convert string '" ^ s ^ "' to int!")

val strings = readInput "input.txt"
val ints = map intFromString strings
val result = foldl op+ 0 ints

