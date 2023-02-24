import java.io.File
import java.io.InputStream

fun main() {
    val inputStream: InputStream = File("../day1.txt").inputStream()
    val lineList = mutableListOf<Int>(0)

    inputStream.bufferedReader().forEachLine {
        if (it == "") {
            lineList.add(0)
        } else {
            lineList[lineList.lastIndex] += it.toInt()
        }
    }

    lineList.sortDescending()

    println(lineList[0])
    println(lineList[0] + lineList[1] + lineList[2])
}
