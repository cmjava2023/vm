package org.cmjava2023;

public class Main {
    public static void main(String[] args) throws Throwable {
        try {
            throw new Throwable("Oops");
        } finally {
            System.out.println("anyway");
        }
    }
}

