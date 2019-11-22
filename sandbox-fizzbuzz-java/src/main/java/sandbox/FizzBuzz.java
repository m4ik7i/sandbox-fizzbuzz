package sandbox;

import java.util.stream.IntStream;

public class FizzBuzz {

    public static void main(String[] args) {
        int n = Integer.parseInt(args[0]);

        IntStream.rangeClosed(1, n).forEach((i) -> {
            System.out.println(fizzbuzz(i));
        });
    }

    public static String fizzbuzz(int n) {
        if (n % 3 == 0 && n % 5 != 0) {
            return "Fizz";
        } else if (n % 5 == 0 && n % 3 != 0) {
            return "Buzz";
        } else if (n % 3 == 0 && n % 5 == 0) {
            return "FizzBuzz";
        } else {
            return Integer.valueOf(n).toString();
        }
    }

}
