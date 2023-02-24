import java.io.File
import java.io.InputStream
import kotlin.io.*

// XYZ is you
// A & X Rock     1
// B & Y Paper    2
// C & Z Scissors 3

const val rock = 1
const val paper = 2
const val scissors = 3

const val loss = 0
const val drawer = 3
const val win = 6

fun main() {
    var first_score = 0
    var second_score = 0

    val inputStream: InputStream = File("../day2.txt").inputStream()

    inputStream.bufferedReader().forEachLine {
        when (it) {
            // Rock
            "A X" -> first_score += rock + drawer // rock rock
            "A Y" -> first_score += paper + win // rock paper
            "A Z" -> first_score += scissors + loss // rock Scissors

            // Paper
            "B X" -> first_score += rock + loss // paper rock
            "B Y" -> first_score += paper + drawer // paper paper
            "B Z" -> first_score += scissors + win // paper Scissors

            // Scissors
            "C X" -> first_score += rock + win // scissors rock
            "C Y" -> first_score += paper + loss // scissors paper
            "C Z" -> first_score += scissors + drawer // scissors Scissors
            else -> {
                println("Error '$it'")
            }
        }

        // XYZ is the result of the game
        // X is loss
        // Y is drawer
        // Z is win
        when (it) {
            // Rock
            "A X" -> second_score += scissors + loss // rock rock
            "A Y" -> second_score += rock + drawer // rock paper
            "A Z" -> second_score += paper + win // rock Scissors

            // Paper
            "B X" -> second_score += rock + loss // paper rock
            "B Y" -> second_score += paper + drawer // paper paper
            "B Z" -> second_score += scissors + win // paper scissors

            // Scissors
            "C X" -> second_score += paper + loss // scissors paper
            "C Y" -> second_score += scissors + drawer // scissors scissors
            "C Z" -> second_score += rock + win // scissors rock
            else -> {
                println("Error '$it'")
            }
        }
    }

    println("First score: $first_score")
    println("Second score: $second_score")
}
