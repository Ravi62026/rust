import java.util.Scanner;

public class NumberSequence {
    public static void main(String[] args) {
        int number = 2; // Starting number
        int increment = 6; // Initial increment
        int count = 0; // To track how many times the increment is used

        for (int i = 0; i < 10; i++) { // Generate the first 10 numbers
            System.out.printf("%02d ", number); // Print the number with leading zero if needed
            number += increment;
            count++;

            if (count == 2) { // After using the same increment twice
                increment += 4; // Increase the increment by 4
                count = 0; // Reset the count
            }
        }
    }
}

