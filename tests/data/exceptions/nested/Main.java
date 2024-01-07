package org.cmjava2023;

public class Main {
    public static void main(String[] args) {
        try {
            oops();
        } catch (Throwable e) {
            System.out.println("caught e:");
            System.out.println(e.getMessage());
        } finally {
            System.out.println("anyway");
        }
    }

    public static void oops() throws Throwable {
        throw new Throwable("Oops");
    }
}

