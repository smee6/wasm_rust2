package com.hanyangTest.seo.controller;

import org.springframework.web.bind.annotation.*;
import com.hanyangTest.seo.utils.Factorizer;
import com.hanyangTest.seo.utils.Matrix;

@RestController
@RequestMapping("/")
public class TestController {

    @GetMapping("/hello")
    public String hello() {
        return "Hello, world!";
    }

    @GetMapping("/bye")
    public String bye() {
        return "Goodbye, world!";
    }

    @PostMapping("/mirror")
    public String mirror(@RequestBody String body) {
        return body;
    }

    @GetMapping("/mat")
    public String matrix_multiply() {
        int n = 5000;
        double[][] a = new double[n][n];
        double[][] b = new double[n][n];

        // Initialize matrices with some values
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                a[i][j] = 1.0;
                b[i][j] = 2.0;
            }
        }

        long startTime = System.currentTimeMillis();
        double[][] c = Matrix.multiply(a, b);
        long endTime = System.currentTimeMillis();

        return "Matrix multiplication completed in " + (endTime - startTime) + " milliseconds." + "Result[0][0]: "
                + c[0][0];
    }

    @GetMapping("/factorize")
    public String factorize() {
        long startTime = System.currentTimeMillis();

        for (int i = 0; i < 100; i++) {
            Factorizer.factorize(100000000000031L);
        }

        long duration = System.currentTimeMillis() - startTime;
        return "Completed in: " + duration + " ms";
    }

    @GetMapping("/ping")
    public String ping() {
        long startTime = System.currentTimeMillis();
        int limit = 1000000;
        int count = countPrimes(limit);
        long endTime = System.currentTimeMillis();

        return "Found " + count + " primes up to " + limit + " in " + (endTime - startTime) + " milliseconds.";
    }

    private int countPrimes(int limit) {
        int count = 0;

        for (int num = 2; num <= limit; num++) {
            boolean isPrime = true;
            int maxDivisor = (int) Math.sqrt(num);

            for (int i = 2; i <= maxDivisor; i++) {
                if (num % i == 0) {
                    isPrime = false;
                    break;
                }
            }

            if (isPrime) {
                count++;
            }
        }

        return count;
    }
}
