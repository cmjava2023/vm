package org.cmjava2023;

public class Main {
    public static void main(String[] args) {
        String greeting = objectReturn();
        System.out.println("(main) greeting:");
        System.out.println(greeting);
        objectArg(greeting);

        int num = primitivReturn();
        System.out.println("(main) num:");
        System.out.println(num);
        primitiveArg(num);

        int[] nums = arrayReturn();
        System.out.println("(main) nums:");
        System.out.println(nums);
        arrayArg(nums);

        double d = largePrimitiveReturn();
        System.out.println("(main) d:");
        System.out.println(d);
        largePrimitiveArg(d);
    }

    public static void objectArg(String greeting) {
        System.out.println("(objectArg) greeting:");
        System.out.println(greeting);
    }

    public static void primitiveArg(int num) {
        System.out.println("(primitiveArg) num:");
        System.out.println(num);
    }

    public static void arrayArg(int[] nums) {
        System.out.println("(arrayArg) nums:");
        System.out.println(nums);
    }

    public static void largePrimitiveArg(double d) {
        System.out.println("(largePrimitiveArg) d:");
        System.out.println(d);
    }

    public static String objectReturn() {
        return "Hello World";
    }

    public static int primitivReturn() {
        return 10;
    }

    public static int[] arrayReturn() {
        int[] i = {10, 11};
        return i;
    }

    public static double largePrimitiveReturn() {
        return 10.0;
    }
}

