import java.util.Scanner;

public class sum {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        System.out.println("Enter the number of terms:");
        int n = sc.nextInt();
        int sum = 0;

        int num = n;
        
        while (num > 0) {
            int rem = num % 10;
            sum += rem;
            num /= 10;

            if (num == 0 && sum > 9) {
                num = sum;
                sum = 0;
            }
        }
        System.out.println("Sum of digits: " + sum);
    }
}
