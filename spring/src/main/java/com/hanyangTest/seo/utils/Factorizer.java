package com.hanyangTest.seo.utils;

import java.util.ArrayList;
import java.util.List;

public class Factorizer {

    public static List<Long> factorize(long n) {
        List<Long> factors = new ArrayList<>();
        long d = 2;
        while (n > 1) {
            while (n % d == 0) {
                factors.add(d);
                n /= d;
            }
            d += 1;
            if (d * d > n) {
                if (n > 1) {
                    factors.add(n);
                }
                break;
            }
        }
        return factors;
    }
}
