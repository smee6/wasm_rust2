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
        int n = 1000;
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

        return "Matrix multiplication completed in " + (endTime - startTime) + " milliseconds." + "Result[0][0]: " + c[0][0];
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
}
