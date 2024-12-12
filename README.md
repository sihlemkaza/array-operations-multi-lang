# Array Operations

Demonstrating a variety of array-related operations that can be performed across multiple programming languages. 

Covering different techniques for iterating, transforming, and manipulating arrays.

---

## Languages Covered
- Java
- JavaScript
- Python
- Rust

# Constructs/Statements and Methods Covered
- [for](#for)
- [for...of / Enahanced for Loop](#forof--enahanced-for-loop)
- [while](#while)
- [do...while](#dowhile)
- [forEach](#dowhile)
- [map](#map)
- [filter](#filter)
- [every](#every--all-match)
- [some](#some--any-match)
- [find](#find)
- [reduce](#reduce)


## for 
A looping construct that iterates over a range, sequence, or iterable. It allows performing operations on each element in the sequence.

Examples below iterates over an array of numbers and returns a new array with each element doubled.

**JavaScript**

[docs/Reference/Statements/for 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for)

```JavaScript
const originalArray = [1, 2, 3];
const newArray = [];
for (let i = 0; i < originalArray.length; i++) {
  newArray.push(originalArray[i] * 2);
}

// newArray: [2, 4, 6]
```

**Java**

[docs/javase/for 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/jdk.compiler/com/sun/source/tree/ForLoopTree.html)

```Java
int[] originalArray = {1, 2, 3};
int[] newArray = new int[originalArray.length];

for (int i = 0; i < originalArray.length; i++) {
    newArray[i] = originalArray[i] * 2;
}

// newArray: [2, 4, 6]
```


## for...of / Enahanced for Loop
A construct used to iterate over iterable objects like arrays, strings, or collections.

Examples below iterates over an array of numbers and returns a new array with each element doubled.

**JavaScript**

[docs/Reference/Statements/for...of 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of) 
```JavaScript
const originalArray = [1, 2, 3];
const newArray = [];
for (const value of originalArray) {
  newArray.push(value * 2);
}

// newArray: [2, 4, 6]
```

**Java**

[docs/javase/enhanced-for-loop 🔗](https://docs.oracle.com/javase/specs/jls/se23/html/jls-14.html#jls-14.14.2)

```Java
int[] originalArray = {1, 2, 3};
List<Integer> newArray = new ArrayList<>();
for (int value : originalArray) {
    newArray.add(value * 2);
}

// newArray: [2, 4, 6]
```

## while

 A loop that executes as long as a specified condition evaluates to true.

Examples below iterates over an array of numbers and returns a new array with each element doubled.

**JavaScript**

[docs/Reference/Statements/while 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/while)

```JavaScript
const originalArray = [1, 2, 3];
const newArray = [];
let i = 0;
while (i < originalArray.length) {
  newArray.push(originalArray[i] * 2);
  i++;
}

// newArray: [2, 4, 6]
```

**Java**

[docs/javase/while 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/jdk.compiler/com/sun/source/tree/WhileLoopTree.html)

```Java
int[] originalArray = {1, 2, 3};
List<Integer> newArray = new ArrayList<>();
int i = 0;
while (i < originalArray.length) {
    newArray.add(originalArray[i] * 2);
    i++;
}

// newArray: [2, 4, 6]
```

## do...while

Similar to a `while` loop, but it guarantees that the loop body executes at least once because the condition is evaluated after the loop body.

Examples below iterates over an array of numbers and returns a new array with each element doubled.

**JavaScript**

[docs/Reference/Statements/do...while 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/do...while)

```JavaScript
const originalArray = [1, 2, 3];
const newArray = [];
let i = 0;
do {
  newArray.push(originalArray[i] * 2);
  i++;
} while (i < originalArray.length);

// newArray: [2, 4, 6]
```

**Java**

[docs/javase/do-while 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/jdk.compiler/com/sun/source/tree/DoWhileLoopTree.html)

```Java
int[] originalArray = {1, 2, 3};
List<Integer> newArray = new ArrayList<>();
int i = 0;
do {
    newArray.add(originalArray[i] * 2);
    i++;
} while (i < originalArray.length);
```

## forEach

An array method that iterates over each element in a collection or array and applies a provided function or operation to each element. 

Examples below iterates over an array of numbers and returns a new array with each element doubled.

**JavaScript**

[docs/Array.prototype.forEach() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach)

```JavaScript
const originalArray = [1, 2, 3, 4];
const newArray = [];
originalArray.forEach((value) => {
  newArray.push(value * 2);
});

// newArray: [2, 4, 6, 8]
```

## map

An array method that transforms each element in an array by applying a provided function, returning a new array with the transformed elements.

Examples below iterates over an array of numbers and returns a new array with each element doubled.

**JavaScript**

[docs/Array.prototype.map() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map)

```JavaScript
const originalArray = [1, 2, 3, 4];
const result = originalArray.map((value) => value * 2);

// result: [2, 4, 6, 8]
```

**Java**

[docs/javase/map 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#map(java.util.function.Function))

```Java
int[] originalArray = {1, 2, 3, 4};
int[] newArray = Arrays.stream(originalArray)
    .map(value -> value * 2)
    .toArray();
// result (Array): [2, 4, 6, 8]
```
```Java
int[] originalArray = {1, 2, 3, 4};
List<Integer> newArray = Arrays.stream(originalArray)
    .map(value -> value * 2)
    .boxed()
    .collect(Collectors.toList());
// result (List): [2, 4, 6, 8]
```

## filter

Creates a new array containing only elements that satisfy a given condition or predicate function.

Examples below iterate over an array and return a new array with positive numbers.

**JavaScript**

[docs/Array.prototype.filter() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter)

```JavaScript
const originalArray = [1, -2, 3, 4, -5];
const result = originalArray.filter((value) => value > 0);

// result: [1, 3, 4]
```

**Java**

[docs/javase/filter 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#filter(java.util.function.Predicate))

```Java
int[] originalArray = {1, -2, 3, 4, -5};
int[] result = Arrays.stream(originalArray)
    .filter(value -> value > 0)
    .toArray();
```

## every / all match

Checks if all elements in an array satisfy a given condition, returning a boolean value.

- First example below checks if all array elements are even numbers.
- Second example below checks if all array elements are greater than 100.

**JavaScript**

[docs/Array.prototype.every() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every)

```JavaScript
const originalArray = [2, 4, 6];
const result1 = originalArray.every((value) => value % 2 === 0);
const result2 = originalArray.every((value) => value > 100);

// result1: true
// result2: false
```

**Java**

[docs/javase/allMatch 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#allMatch(java.util.function.Predicate))

```Java
int[] originalArray = {2, 4, 6};
boolean result1 = Arrays.stream(originalArray)
    .allMatch(value -> value % 2 == 0);
boolean result2 = Arrays.stream(originalArray)
    .allMatch(value -> value > 100);

// result1: true
// result2: false
```

## some / any match

Checks if at least one element in an array satisfies a given condition, returning a boolean value.

- First example below checks if the array contains any even numbers.
- Second example below checks if the array contains any number greater than 100.

**JavaScript**

[docs/Array.prototype.some() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some)

```JavaScript
const originalArray = [1, 3, 5, 6];
const result1 = originalArray.some((value) => value % 2 === 0);
const result2 = originalArray.some((value) => value > 100);

// result1: true
// result2: false
```

**Java**

[docs/javase/anyMatch 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#anyMatch(java.util.function.Predicate))

```Java
int[] originalArray = {1, 3, 5, 6};
boolean result1 = Arrays.stream(originalArray)
    .anyMatch(value -> value % 2 == 0);
boolean result2 = Arrays.stream(originalArray)
    .anyMatch(value -> value > 100);

// result1: true
// result2: false
```

# find

 Returns the first element in an array that satisfies a specified condition. If no element satisfies the condition, it returns a default or `null`/`undefined` equivalent.

 Examples below demonstrate locate the first number greater than 0 and divisible by 2.

**JavaScript**

[docs/Array.prototype.find() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find)

```JavaScript
const originalArray = [-1, 3, 6, 7, 8];
const result = originalArray.find((value) => value > 0 && value % 2 === 0);

// result: 6
```

**Java**

[docs/javase/findFirst 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#findFirst())

```Java
int[] originalArray = {-1, 3, 6, 7, 8};
OptionalInt result = Arrays.stream(originalArray)
    .filter(value -> value > 0 && value % 2 == 0)
    .findFirst();

// result: 6
```

## reduce

Applies a function to each element of an array to reduce it to a single accumulated value.


### Reduce Example 1: Sum all numbers in an array

**JavaScript**

[docs/Array.prototype.reduce() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce)

```JavaScript
const originalArray = [1, 2, 3, 4];
const result = originalArray.reduce(
    (accumulator, currentValue) => accumulator + currentValue, 0);

// result: 10
```

**Java**

[docs/javase/reduce 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#reduce(T,java.util.function.BinaryOperator))

```Java
int[] originalArray = {1, 2, 3, 4};
int result = Arrays.stream(originalArray)
    .reduce(0, Integer::sum);

// result: 10
```

### Reduce Example 2: Find the maximum value in an array

**JavaScript**
```JavaScript
const originalArray = [3, 7, 2, 9, 5];
const result = originalArray.reduce(
        (max, currentValue) => (currentValue > max ? currentValue : max), 
        -Infinity
    );

// result: 9
```

**Java**
```Java
int[] originalArray = {3, 7, 2, 9, 5};
int result = Arrays.stream(originalArray)
    .reduce(Integer.MIN_VALUE, Integer::max);

// result: 9
```

### Reduce Example 3: Flatten a nested array

**JavaScript**
```JavaScript
const originalArray = [[1, 2], [3, 4], [5]];
const result = originalArray.reduce(
        (flatArray, currentArray) => flatArray.concat(currentArray), []
    );

// result: [1, 2, 3, 4, 5]
```

**Java**
```Java
int[][] originalArray = {{1, 2}, {3, 4}, {5}};
int[] result = Arrays.stream(originalArray)
    .flatMapToInt(Arrays::stream)
    .toArray();
    
// result: [1, 2, 3, 4, 5]
```