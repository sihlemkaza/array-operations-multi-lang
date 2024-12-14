# Array Operations

Demonstrating a variety of array-related operations for iterating, transforming, and manipulating arrays.

---

## Languages Covered
- Java
- JavaScript
- Python
- Rust

## Project Examples
This repository provides examples of array operations implemented in various programming languages. The supported languages include 
Java, JavaScript, Python, and Rust. Follow the instructions below to run the code for each language.

### Prerequisites
To run the code examples, ensure that the required tools for each language are installed on your system:
 - **Java**: JDK 8 or higher
 - **JavaScript**: Node.js (v14 or higher recommended)
 - **Python**: Python 3.7 or higher
 - **Rust**: Rust 1.53 or higher

### Java
Steps 1 can be skipped on Java 11 or higher.
1. From the root directory, compile the `ArrayOperations.java` file
```bash
javac /examples/java/ArrayOperations.java
```
2. Run the compiled program:
```bash
java /examples/java/ArrayOperations.java
```

### JavaScript
From the root directory, run the `array-operations.js` file using Node.js:
 ```bash
node examples/javascript/array-operations.js
 ```

### Python
From the root directory, run the `array_operations.py` file:
```bash
python examples/python/array_operations.py
```

### Rust
1. Navigate to the `examples/rust` directory
```bash
cd examples/rust
```

2. Build and run the Rust examples using Cargo
```bash
cargo run
```

# Constructs/Statements and Methods Covered
- [for range](#for-range)
- [for...of | Enahanced for Loop | for...in](#forof--enahanced-for-loop--forin)
- [while](#while)
- [do...while](#dowhile)
- [Infinite Loop](#infinite-loop)
- [for each](#for-each)
- [map](#map)
- [filter](#filter)
- [every | all match](#every--all-match)
- [some | any match](#some--any-match)
- [find](#find)
- [reduce | fold](#reduce--fold)

## for range
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

console.log('newArray: ', newArray); // newArray: [2, 4, 6]
```

**Java**

[docs/javase/for 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/jdk.compiler/com/sun/source/tree/ForLoopTree.html)

```Java
int[] originalArray = {1, 2, 3};
int[] newArray = new int[originalArray.length];

for (int i = 0; i < originalArray.length; i++) {
    newArray[i] = originalArray[i] * 2;
}

System.out.println("newArray: " + Arrays.toString(newArray));
// newArray: [2, 4, 6]
```

**Rust**

[docs/rust-lang/for 🔗](https://doc.rust-lang.org/std/keyword.for.html)

```Rust
let original_array = vec![1, 2, 3];
let mut new_array = Vec::new();
for i in 0..original_array.len() {
    new_array.push(original_array[i] * 2);
}

println!("new_array: {:?}", new_array); // new_array: [2, 4, 6]
```

**Python**

[docs/python/for 🔗](https://docs.python.org/3/tutorial/controlflow.html#for-statements)

```Python
original_array = [1, 2, 3]
new_array = []
for i in range(len(original_array)):
    new_array.append(original_array[i] * 2)

print(f"{new_array=}") # new_array=[2, 4, 6]
```


## for...of | Enahanced for Loop | for...in
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

console.log('newArray: ', newArray); // newArray: [2, 4, 6]
```

**Java**

[docs/javase/enhanced-for-loop 🔗](https://docs.oracle.com/javase/specs/jls/se23/html/jls-14.html#jls-14.14.2)

```Java
int[] originalArray = {1, 2, 3};
List<Integer> newArray = new ArrayList<>();
for (int value : originalArray) {
    newArray.add(value * 2);
}

System.out.println("newArray: " + newArray); // newArray: [2, 4, 6]
```

**Rust**

[docs/rust-lang/for 🔗](https://doc.rust-lang.org/std/keyword.for.html)

```Rust
let original_array = vec![1, 2, 3];
let mut new_array = Vec::new();
for &value in &original_array {
    new_array.push(value * 2);
}

println!("new_array: {:?}", new_array); // new_array: [2, 4, 6]
```

**Python**

[docs/python/for 🔗](https://docs.python.org/3/tutorial/controlflow.html#for-statements)

```Python
original_array = [1, 2, 3]
new_array = []
for value in original_array:
    new_array.append(value * 2)

print(f"{new_array=}") # new_array=[2, 4, 6]
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

console.log('newArray: ', newArray); // newArray: [2, 4, 6]
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

System.out.println("newArray: " + newArray); // newArray: [2, 4, 6]
```

**Rust**

[docs/rust-lang/while 🔗](https://doc.rust-lang.org/std/keyword.while.html)

```Rust
let original_array = vec![1, 2, 3];
let mut new_array = Vec::new();
let mut i = 0;
while i < original_array.len() {
    new_array.push(original_array[i] * 2);
    i += 1;
}

println!("new_array: {:?}", new_array); // new_array: [2, 4, 6]
```

**Python**

[docs/python-wiki/while 🔗](https://wiki.python.org/moin/WhileLoop)

```Python
original_array = [1, 2, 3]
new_array = []
i = 0
while i < len(original_array):
    new_array.append(original_array[i] * 2)
    i += 1

print(f"{new_array=}") # new_array=[2, 4, 6]
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

console.log('newArray: ', newArray); // newArray: [2, 4, 6]
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

System.out.println("newArray: " + newArray); // newArray: [2, 4, 6]
```

## Infinite Loop

The `loop` in Rust is used to create an infinite loop. It will keep executing until explicitly stopped
with a break or an external condition (like a system signal). You can also break with a return value
that can be assigned to a variable e.g. `let result = loop { inner_logic... break 5; }`.

Examples below iterates over an array of numbers and returns a new array with each element doubled.

**Rust**

[docs/rust-lang/loop 🔗](https://doc.rust-lang.org/std/keyword.loop.html)

```Rust
let original_array = vec![1, 2, 3];
let mut new_array = Vec::new();
let mut i = 0;
loop {
    new_array.push(original_array[i] * 2);
    i += 1;
    if i >= original_array.len() {
        break;
    }
}

println!("new_array: {:?}", new_array); // new_array: [2, 4, 6]
```

## for each

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

console.log('newArray: ', newArray); // new_array: [2, 4, 6, 8]
```

**Rust**

[docs/rust-lang/for_each 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each)

```Rust
let original_array = vec![1, 2, 3, 4];
let mut new_array = Vec::new();
original_array.iter().for_each(|&value| {
    new_array.push(value * 2);
});

println!("new_array: {:?}", new_array); // new_array: [2, 4, 6, 8]
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

System.out.println("result (Array): " + Arrays.toString(newArray));
// result (Array): [2, 4, 6, 8]
```

```Java
int[] originalArray = {1, 2, 3, 4};
List<Integer> newArray = Arrays.stream(originalArray)
    .map(value -> value * 2)
    .boxed()
    .collect(Collectors.toList());

System.out.println("result (List): " + newArray);
// result (List): [2, 4, 6, 8]
```

**Rust**

[docs/rust-lang/map 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

```Rust
let original_array = vec![1, 2, 3, 4];
let new_array: Vec<_> = original_array.iter()
    .map(|&value| value * 2).collect();

println!("new_array: {:?}", new_array); // new_array: [2, 4, 6, 8]
```
**Python**

[docs/python/map 🔗](https://docs.python.org/3/library/functions.html#map)

```Python
original_array = [1, 2, 3, 4]
new_array = list(map(lambda value: value * 2, original_array))

print(f"{new_array=}") # new_array=[2, 4, 6, 8]
```

## filter

Creates a new array containing only elements that satisfy a given condition or predicate function.

Examples below iterate over an array and return a new array with positive numbers.

**JavaScript**

[docs/Array.prototype.filter() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter)

```JavaScript
const originalArray = [1, -2, 3, 4, -5];
const newArray = originalArray.filter((value) => value > 0);

console.log('newArray: ', newArray); // newArray: [1, 3, 4]
```

**Java**

[docs/javase/filter 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#filter(java.util.function.Predicate))

```Java
int[] originalArray = {1, -2, 3, 4, -5};
int[] newArray = Arrays.stream(originalArray)
    .filter(value -> value > 0)
    .toArray();

System.out.println("newArray: " + Arrays.toString(newArray));
// newArray: [1, 3, 4]
```

**Rust**

[docs/rust-lang/filter 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

```Rust
let original_array = vec![1, -2, 3, 4, -5];
let new_array: Vec<_> = original_array.iter()
    .filter(|&&value| value > 0).cloned().collect();

println!("new_array: {:?}", new_array); // new_array: [1, 3, 4]
```

**Python**

[docs/python/filter 🔗](https://docs.python.org/3/library/functions.html#filter)

```Python
original_array = [1, -2, 3, 4, -5]
new_array = list(filter(lambda value: value > 0, original_array))

print(f"{new_array=}") # new_array=[1, 3, 4]
```

## every | all match

Checks if all elements in an array satisfy a given condition, returning a boolean value.

- First example below checks if all array elements are even numbers.
- Second example below checks if all array elements are greater than 100.

**JavaScript**

[docs/Array.prototype.every() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every)

```JavaScript
const originalArray = [2, 4, 6];
const result1 = originalArray.every((value) => value % 2 === 0);
const result2 = originalArray.every((value) => value > 100);

console.log({ result1, result2 }); // { result1: true, result2: false }
```

**Java**

[docs/javase/allMatch 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#allMatch(java.util.function.Predicate))

```Java
int[] originalArray = {2, 4, 6};
boolean result1 = Arrays.stream(originalArray)
    .allMatch(value -> value % 2 == 0);
boolean result2 = Arrays.stream(originalArray)
    .allMatch(value -> value > 100);

System.out.println("result1: " + result1 + ", result2: " + result2);
// result1: true, result2: false
```

**Rust**

[docs/rust-lang/all 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all)

```Rust
let original_array = vec![2, 4, 6];
let result1 = original_array.iter().all(|&value| value % 2 == 0);
let result2 = original_array.iter().all(|&value| value > 100);

println!("result1: {:?}, result2: {:?}", result1, result2); 
// result1: true, result2: false
```

**Python**

[docs/python/all 🔗](https://docs.python.org/3/library/functions.html#all)

```Python
original_array = [2, 4, 6]
result1 = all(value % 2 == 0 for value in original_array)
result2 = all(value > 100 for value in original_array)

print(f"{result1=}, {result2=}") # result=True, result2=False
```

## some | any match

Checks if at least one element in an array satisfies a given condition, returning a boolean value.

- First example below checks if the array contains any even numbers.
- Second example below checks if the array contains any number greater than 100.

**JavaScript**

[docs/Array.prototype.some() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some)

```JavaScript
const originalArray = [1, 3, 5, 6];
const result1 = originalArray.some((value) => value % 2 === 0);
const result2 = originalArray.some((value) => value > 100);

console.log({ result1, result2 }); // { result1: true, result2: false }
```

**Java**

[docs/javase/anyMatch 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#anyMatch(java.util.function.Predicate))

```Java
int[] originalArray = {1, 3, 5, 6};
boolean result1 = Arrays.stream(originalArray)
    .anyMatch(value -> value % 2 == 0);
boolean result2 = Arrays.stream(originalArray)
    .anyMatch(value -> value > 100);

System.out.println("result1: " + result1 + ", result2: " + result2);
// result1: true, result2: false
```

**Rust**

[docs/rust-lang/any 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)

```Rust
let original_array = vec![1, 3, 5, 6];
let result1 = original_array.iter().any(|&value| value % 2 == 0);
let result2 = original_array.iter().any(|&value| value > 100);

println!("result1: {:?}, result2: {:?}", result1, result2); 
// result1: true, result2: false
```

**Python**

[docs/python/any 🔗](https://docs.python.org/3/library/functions.html#any)

```Python
original_array = [1, 3, 5, 6]
result1 = any(value % 2 == 0 for value in original_array)
result2 = any(value > 100 for value in original_array)

print(f"{result1=}, {result2=}") # result=True, result2=False
```

# find

 Returns the first element in an array that satisfies a specified condition. If no element satisfies the condition, it returns a default or `null`/`undefined` equivalent.

 Examples below demonstrate locating the first number greater than 0 and is even.

**JavaScript**

[docs/Array.prototype.find() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find)

```JavaScript
const originalArray = [-1, 3, 6, 7, 8];
const result = originalArray.find((value) => value > 0 && value % 2 === 0);

console.log('result:', result); // result: 6
```

**Java**

[docs/javase/findFirst 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#findFirst())

```Java
int[] originalArray = {-1, 3, 6, 7, 8};
OptionalInt result = Arrays.stream(originalArray)
    .filter(value -> value > 0 && value % 2 == 0)
    .findFirst();

System.out.println("result: " + result.getAsInt()); // result: 6
```

**Rust**

[docs/rust-lang/find 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)

```Rust
let original_array = vec![-1, 3, 6, 7, 8];
let result = original_array.iter()
    .find(|&&value| value > 0 && value % 2 == 0);

println!("result: {:?}", result); // result: Some(6)
```
**Python**

[docs/python/next 🔗](https://docs.python.org/3/library/functions.html#next)

```Python
 original_array = [-1, 3, 6, 7, 8]
result = next(
    (value for value in original_array if value > 0 and value % 2 == 0), 
    None
)

print(f"{result=}") # result=6
```

## reduce | fold
Both the `reduce` and `fold` functions apply a function to each element of an array/collection and returns a single accumulated value.
- `reduce` - an initial accumulator value is **optional**. First element from collection is used if not supplied.
    - **in Rust**, no initial value is accepted, works on existing collection elements.
- `fold` - *(Rust only)* initial accumulator value is **required**.


### Reduce Example 1: Sum all numbers in an array

**JavaScript**

[docs/Array.prototype.reduce() 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce)

```JavaScript
const originalArray = [1, 2, 3, 4];
const result = originalArray.reduce(
    (accumulator, currentValue) => accumulator + currentValue, 0);

console.log('result:', result); // result: 10
```

**Java**

[docs/javase/reduce 🔗](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/stream/Stream.html#reduce(T,java.util.function.BinaryOperator))

```Java
int[] originalArray = {1, 2, 3, 4};
int result = Arrays.stream(originalArray)
    .reduce(0, Integer::sum);

System.out.println("result: " + result); // result: 10
```

**Rust**

[docs/rust-lang/reduce 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce)

```Rust
let original_array = vec![1, 2, 3, 4];
let result: i32 = original_array
    .iter().cloned()
    .reduce(|accumulator , current| accumulator  + current)
    .unwrap_or(0);

println!("result: {:?}", result); // result: 10
```

[docs/rust-lang/fold 🔗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
```Rust
let original_array = vec![1, 2, 3, 4];
let result: i32 = original_array.iter()
    .fold(0, |accumulator, current| accumulator + current);

println!("result: {:?}", result); // result: 10
```

**Python**

[docs/python/reduce 🔗](https://docs.python.org/3/library/functools.html#functools.reduce)

```Python
original_array = [1, 2, 3, 4]
result = reduce(
    lambda acc, current: acc + current, 
    original_array, 
    0
)

print(f"{result=}") # result=10
```

### Reduce Example 2: Find the maximum value in an array

**JavaScript**
```JavaScript
const originalArray = [3, 7, 2, 9, 5];
const result = originalArray.reduce(
        (max, currentValue) => (currentValue > max ? currentValue : max), 
        -Infinity
    );

console.log('result:', result); // result: 9
```

**Java**
```Java
int[] originalArray = {3, 7, 2, 9, 5};
int result = Arrays.stream(originalArray)
    .reduce(Integer.MIN_VALUE, Integer::max);

System.out.println("result: " + result); // result: 9
```

**Rust**

```Rust
let original_array = vec![3, 7, 2, 9, 5];
let result = original_array.iter().cloned().reduce(i32::max).unwrap_or(i32::MIN);

println!("result: {:?}", result); // result: 9
```

```Rust
let original_array = vec![3, 7, 2, 9, 5];
let result = original_array.iter().cloned().fold(i32::MIN, i32::max);

println!("result: {:?}", result); // result: 9
```

**Python**
```Python
original_array = [3, 7, 2, 9, 5]
result = reduce(
    lambda max_val, current: current if current > max_val else max_val, 
    original_array, 
    -float('inf')
)

print(f"{result=}") # result=9
```

### Reduce Example 3: Flatten a nested array

**JavaScript**
```JavaScript
const originalArray = [[1, 2], [3, 4], [5]];
const newArray = originalArray.reduce(
        (flatArray, currentArray) => flatArray.concat(currentArray), 
        []
    );

console.log('newArray:', newArray); // newArray: [1, 2, 3, 4, 5]
```

**Java**
```Java
int[][] originalArray = {{1, 2}, {3, 4}, {5}};
int[] newArray = Arrays.stream(originalArray)
    .flatMapToInt(Arrays::stream)
    .toArray();
    
System.out.println("newArray: " + Arrays.toString(newArray)); 
// newArray: [1, 2, 3, 4, 5]
```

**Rust**
```Rust
let original_array = vec![vec![1, 2], vec![3, 4], vec![5]];
let new_array: Vec<_> = original_array
    .iter().cloned()
    .reduce(|mut accumulator , current| {
        accumulator.extend(current);
        accumulator
    })
    .unwrap_or_else(Vec::new);

println!("new_array: {:?}", new_array); // new_array: [1, 2, 3, 4, 5]
```

```Rust
let original_array = vec![vec![1, 2], vec![3, 4], vec![5]];
let new_array: Vec<_> = original_array
    .iter().cloned()
    .fold(Vec::new(),|mut accumulator , current| {
        accumulator.extend(current);
        accumulator
    });

println!("new_array: {:?}", new_array); // new_array: [1, 2, 3, 4, 5]
```

**Python**
```Python
original_array = [[1, 2], [3, 4], [5]]
new_array = reduce(
    lambda flat, current: flat + current, 
    original_array, 
    []
)

print(f"{new_array=}") # new_array=[1, 2, 3, 4, 5]
```