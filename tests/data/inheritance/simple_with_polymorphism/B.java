public class B extends A {
    public void doStuff() {
        super.doStuff();
        System.out.println("(B) doStuff()");
        doOtherStuff();
    }
}

