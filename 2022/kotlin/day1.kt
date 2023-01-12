import kotlin.io.*
import java.io.File

fun main() {
    val file = File("day1.txt")
    val contents = file.readText()
    println(contents)
}
