package org.cmjava2023;

public class Main {
    int num;

    public Main(int num) {
        this.num = num;
    }

    public static void main(String[] args) {
        Main m = new Main(10);
        m.doStuff();
    }

    public void doStuff() {
        System.out.println(this.num);
    }
}
