public class Main {
    public static void main(String[] args) {
        try {
            throw new A("Oops");
        } catch (Throwable e) {
            System.out.println("caught e:");
            System.out.println(e.getMessage());
        }
        try {
            try {
                throw new Throwable("Huh?");
            } catch (A a) {
                System.out.println("caught a:");
                System.out.println(a.getMessage());
            }
        } catch (Throwable e) {
            System.out.println("did not catch e:");
            System.out.println(e.getMessage());
        }
    }
}
