import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        System.out.println("Enter the number of prime numbers to sum:");
        int n = sc.nextInt();

        int count = 0; // Count of prime numbers found
        int num = 2; // Number to check for primality
        int sum = 0; // Sum of prime numbers

        System.out.println("Prime numbers are:");
        while (count < n) {
            if (isPrime(num)) {
                System.out.print(num + " ");
                sum += num;
                count++;
            }
            num++;
        }

        System.out.println("\nSum of the first " + n + " prime numbers is: " + sum);

        sc.close(); // Close the scanner to prevent resource leak
    }

    // Function to check if a number is prime
    public static boolean isPrime(int num) {
        if (num <= 1)
            return false;
        for (int i = 2; i <= Math.sqrt(num); i++) {
            if (num % i == 0)
                return false;
        }
        return true;
    }
}