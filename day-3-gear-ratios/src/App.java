import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * Sample input:
 *    467..114..
 *    ...*......
 *    ..35..633.
 *    ......#...
 *    617*......
 *    .....+.58.
 *    ..592.....
 *    ......755.
 *    ...$.*....
 *    .664.598..
 *
 * Pseudo-code
 *  prevLine, currLine = input
 *  ----> ....***.....
 *  ----> ....*11.....
 *  ----> 12..**......
 *
 * Lookup
 *      11: true
 *      12: false
 */

public class App {

    public static class Number {
        private int startIdx;
        private int endIdx;
        private int number;

        /**
         * Representation of a Number
         *
         * @param startIdx - inclusive start index
         * @param endIdx - inclusive end index
         * @param number - integer number value
         */
        public Number(int startIdx, int endIdx, int number) {
            this.startIdx = startIdx;
            this.endIdx = endIdx;
            this.number = number;
        }

        public int getNumber() {
            return number;
        }

        public boolean adjacent(int index) {
            return (this.startIdx - 1) <= index && index <= (this.endIdx + 1);
        }

        public boolean overlaps(int index) {
            return this.startIdx <= index && index <= this.endIdx;
        }

        public boolean attached(int index) {
            return (this.startIdx - 1) == index || index == (this.endIdx + 1);
        }

        // ....*****...
        // ....*123*...
        // ....*****...
    }

    private static List<Number> extractNumbers(String line) {
        List<Number> numbers = new ArrayList<Number>();

        // Identify locations of numbers
        Pattern pattern = Pattern.compile("[0-9]+");
        Matcher matcher = pattern.matcher(line);
        while (matcher.find())
            numbers.add(
                new Number(
                    matcher.start(),
                    matcher.end() - 1,
                    Integer.parseInt(matcher.group())));

        return numbers;
    }

    private static List<Integer> extractSymbols(String line) {
        List<Integer> indices = new ArrayList<Integer>();

        // Identify indexes of numbers
        Pattern pattern = Pattern.compile("[^0-9.]");
        Matcher matcher = pattern.matcher(line);
        while (matcher.find())
            indices.add(matcher.start());

        return indices;
    }

    private static boolean checkLine(Number number, String line) {
        // Get locations of all the symbols
        List<Integer> symbols = extractSymbols(line);

        // Check if any symbol overlaps the number - since horizontal, check horizontal overlap
        for (int symbol : symbols) {
            if (number.adjacent(symbol))
                return true;
        }

        return false;
    }

    public static void main(String[] args) throws FileNotFoundException {
        Scanner inputFile = new Scanner(new File("./src/input.txt"));

        // Start line scan loop
        String previousLine = "", currentLine = "", nextLine = "";

        if (inputFile.hasNextLine()) {
            currentLine = inputFile.nextLine();
        }

        int total = 0;
        int lines = 1;

        // Assumption: input file has at least 3 lines
        while (inputFile.hasNextLine()) {
            lines++;
            nextLine = inputFile.nextLine();

            // From current line, get all the numbers
            List<Number> numbers = extractNumbers(currentLine);

            // For each number, check if the current line, previous line, or next line triggers it
            for (Number number : numbers) {
                if (checkLine(number, previousLine)
                    || checkLine(number, currentLine)
                    || checkLine(number, nextLine)
                ) {
                    // Add each triggered number to the total
                    // System.out.println("Adding " + number.getNumber());
                    total += number.getNumber();
                }
            }

            previousLine = currentLine;
            currentLine = nextLine;
        }

        // Loop ends if nextLine already consumes the last line - repeat
        List<Number> numbers = extractNumbers(currentLine);
        for (Number number : numbers) {
            if (checkLine(number, previousLine)
                || checkLine(number, currentLine)
            ) {
                total += number.getNumber();
            }
        }

        System.out.println("Total: " + total);
        System.out.println("Lines: " + lines);

        inputFile.close();
    }
}
