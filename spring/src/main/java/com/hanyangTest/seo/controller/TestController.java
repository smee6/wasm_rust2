package com.hanyangTest.seo.controller;

import org.springframework.web.bind.annotation.*;
import com.hanyangTest.seo.utils.Factorizer;

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

    @GetMapping("/factorize")
    public String factorize() {
        long startTime = System.currentTimeMillis();

        for (int i = 0; i < 1000; i++) {
            Factorizer.factorize(10000000000031L);
        }

        long duration = System.currentTimeMillis() - startTime;
        return "Completed in: " + duration + " ms";
    }
}
