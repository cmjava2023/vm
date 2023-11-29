package org.cmjava2023;

public class Main {
    public static void main(String[] args) {
        int i = 10;
        // i2b
        byte b = (byte) i;
        // i2c
        char c = (char) i;
        // i2d
        double d = (double) i;
        // i2f
        float f = (float) i;
        // i2l
        long l = (long) i;
        // i2s
        short s = (short) i;
        // iload without index
        int force_load = s;
    }
}
