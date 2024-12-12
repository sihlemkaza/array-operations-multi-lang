import java.util.*;
import java.util.stream.*;

public class ArrayOperations {
    private static void printResult(String name, Object originalArray, Object result) {
        System.out.println(name + ": originalArray = " + arrayToString(originalArray)
            + ", result = " + arrayToString(result));
    }

    private static String arrayToString(Object item) {
        if (item instanceof int[]) {
            return Arrays.toString((int[]) item);
        } else if (item instanceof int[][]) {
            return Arrays.deepToString((int[][]) item);
        } else if (item instanceof List) {
            return item.toString();
        } else if (item instanceof OptionalInt) {
            return ((OptionalInt) item).isPresent()
                ? String.valueOf(((OptionalInt) item).getAsInt()) : "empty";
        } else {
            return String.valueOf(item);
        }
    }
    
    public static void main(String[] args) {
        {
            int[] originalArray = {1, 2, 3};
            int[] newArray = new int[originalArray.length];
            for (int i = 0; i < originalArray.length; i++) {
                newArray[i] = originalArray[i] * 2;
            }
            printResult("for", originalArray, newArray);
        }

        {
            int[] originalArray = {1, 2, 3};
            List<Integer> newArray = new ArrayList<>();
            for (int value : originalArray) {
                newArray.add(value * 2);
            }
            printResult("for-each", originalArray, newArray);
        }

        {
            int[] originalArray = {1, 2, 3};
            List<Integer> newArray = new ArrayList<>();
            int i = 0;
            while (i < originalArray.length) {
                newArray.add(originalArray[i] * 2);
                i++;
            }
            printResult("while", originalArray, newArray);
        }

        {
            int[] originalArray = {1, 2, 3};
            List<Integer> newArray = new ArrayList<>();
            int i = 0;
            do {
                newArray.add(originalArray[i] * 2);
                i++;
            } while (i < originalArray.length);
            printResult("do...while", originalArray, newArray);
        }

        {
            int[] originalArray = {1, 2, 3, 4};
            List<Integer> newArray = Arrays.stream(originalArray)
                .map(value -> value * 2)
                .boxed()
                .collect(Collectors.toList());
            printResult("map to List", originalArray, newArray);
        }

        {
            int[] originalArray = {1, 2, 3, 4};
            int[] newArray = Arrays.stream(originalArray)
                .map(value -> value * 2)
                .toArray();
            printResult("map to Array", originalArray, newArray);
        }

        {
            int[] originalArray = {1, -2, 3, 4, -5};
            int[] result = Arrays.stream(originalArray)
                .filter(value -> value > 0)
                .toArray();
            printResult("filter", originalArray, result);
        }

        {
            int[] originalArray = {-1, 3, 6, 7, 8};
            OptionalInt result = Arrays.stream(originalArray)
                .filter(value -> value > 0 && value % 2 == 0)
                .findFirst();
            printResult("find", originalArray, result);
        }

        {
            int[] originalArray = {2, 4, 6};
            boolean result = Arrays.stream(originalArray)
                .allMatch(value -> value % 2 == 0);
            printResult("every", originalArray, result);
        }

        {
            int[] originalArray = {1, 3, 5, 6};
            boolean result = Arrays.stream(originalArray)
                .anyMatch(value -> value % 2 == 0);
            printResult("some", originalArray, result);
        }

        {
            int[] originalArray = {1, 2, 3, 4};
            int result = Arrays.stream(originalArray)
                .reduce(0, Integer::sum);
            printResult("reduce...sumArray", originalArray, result);
        }

        {
            int[] originalArray = {3, 7, 2, 9, 5};
            int result = Arrays.stream(originalArray)
                .reduce(Integer.MIN_VALUE, Integer::max);
            printResult("reduce...maxVal", originalArray, result);
        }

        {
            int[][] originalArray = {{1, 2}, {3, 4}, {5}};
            int[] result = Arrays.stream(originalArray)
                .flatMapToInt(Arrays::stream)
                .toArray();
            printResult("reduce...flattenArray", originalArray, result);
        }
    }
}