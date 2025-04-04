package zarma;

public class Main {
    public static void main(String[] args) {
        long zarma = 5;
        double wow = 10.5;

        System.out.println("ZARMA: " + zarma + " WOW: " + wow);
    }

    public int wow(int z) {
        return z + 1;
    }

    public static long wow(long z) {
        return z + 1;
    }

    public static long wow(short z) {
        return ((long)z) + 1;
    }
}
