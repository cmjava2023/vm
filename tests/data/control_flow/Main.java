package org.cmjava2023;

public class Main {
    public static void main(String[] args) {
        for (int i = 0; i < 100; i++) {
            if (i % 10 == 0) {
                System.out.println("i % 10 == 0");
            }
        }

        int a = 10;
        if (a / 2 == 5) {
            System.out.println("a / 2 == 5");
        } else {
            System.out.println("a / 2 != 5");
        }

        long l = 100;
        if (l / 2 == 5) {
            System.out.println("l / 2 == 5");
        } else {
            System.out.println("l / 2 != 5");
        }

        double d = 10.0;
        while (d < 15.0) {
            d += 1.0;
        }
        System.out.println("d:");
        System.out.println(d);

        float f = 12.0f;
        if (f < 10) {
            System.out.println("f < 10");
        } else if (f > 10) {
            System.out.println("f > 10");
        } else {
            System.out.println("f == 10");
        }

        String s1 = "Hello World";
        String s2 = s1;
        String s3 = "Goodbye";
        if (s1 == s2) {
            System.out.println("s1 == s2");
        } else {
            System.out.println("s1 != s2");
        }
        if (s1 != s3) {
            System.out.println("s1 != s3");
        } else {
            System.out.println("s1 == s3");
        }
    }
}
