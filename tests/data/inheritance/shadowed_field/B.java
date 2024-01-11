public class B extends A {
    public int thing;

    public B(int thing) {
        super(thing * 2);
        this.thing = thing;
    }

    public void printIt() {
        System.out.println("(B) thing");
        System.out.println(this.thing);
        System.out.println("(A) thing");
        System.out.println(super.thing);
    }
}

