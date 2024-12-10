# Array Operations

Demonstrating a variety of array-related operations that can be performed across multiple programming languages. 

Covering different techniques for iterating, transforming, and manipulating arrays.

---

## Languages Covered
  - [JavaScript/TypeScript](#javascripttypescript)
      - [for](#for-loop), [for...of](#forof-loop), [while](#while-loop), [do...while](#dowhile-loop)
      - [map](#map), [forEach](#foreach), [filter](#filter), [some](#some), [every](#every), [find](#find),
        [reduce](#reduce)
  - [Java](#java)
  - [Python](#python)
  - [Rust](#rust)

# JavaScript/TypeScript
## JavaScript/TypeScript Constructs

### For Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for): 
A traditional loop using an index or counter.

```TypeScript
const originalArray = [1, 2, 3];
const newArray = [];
for (let i = 0; i < originalArray.length; i++) {
  newArray.push(originalArray[i] * 2);
}

// newArray: [2, 4, 6]
```

### for...of Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of): 
Iterates over the values of an iterable (e.g., arrays, strings).

```TypeScript
const originalArray = [1, 2, 3];
const newArray = [];
for (const value of originalArray) {
  newArray.push(value * 2);
}

// newArray: [2, 4, 6]
```

### while Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/while): 
Executes a block of code as long as the condition is `true`.

```TypeScript
const originalArray = [1, 2, 3];
const newArray = [];
let i = 0;
while (i < originalArray.length) {
  newArray.push(originalArray[i] * 2);
  i++;
}

// newArray: [2, 4, 6]
```

### do...while Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/do...while):
Similar to `while`, but the code block runs at least once. Runs at least once even if the `do` condition is `false`.

```TypeScript
const originalArray = [1, 2, 3];
const newArray = [];
let i = 0;
do {
  newArray.push(originalArray[i] * 2);
  i++;
} while (i < originalArray.length);

// newArray: [2, 4, 6]
```

## JavaScript/TypeScript Array Methods

### forEach
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach): 
Iterates over an array, executing a callback for each element.

```TypeScript
const originalArray = [1, 2, 3, 4];
const newArray = [];
originalArray.forEach((value) => {
  newArray.push(value * 2);
});

// newArray: [2, 4, 6, 8]
```

### map
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map): 
Creates a new array by applying a function to each element.

```TypeScript
const originalArray = [1, 2, 3, 4];
const result = originalArray.map((value) => value * 2);

// result: [2, 4, 6, 8]
```

### filter
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter): 
Filters elements based on a condition and returns a new array with only the elements that satisfy the condition.

```TypeScript
const originalArray = [1, -2, 3, 4, -5];
const result = originalArray.filter((value) => value > 0);

// result: [1, 3, 4]
```

### every
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every):
Checks if all elements in the array satisfy the condition and returns `true` if they do.

```TypeScript
const originalArray = [2, 4, 6];
const result = originalArray.every((value) => value % 2 === 0);

// result: true
```

### some
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some): 
Checks if at least one element in the array satisfies the condition and returns `true` if it does.

```TypeScript
const originalArray = [1, 3, 5, 6];
const result = originalArray.some((value) => value % 2 === 0);

// result: true
```

### find
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find): 
Returns the first element that satisfies a condition.

```TypeScript
const originalArray = [-1, 3, 6, 7, 8];
const result = originalArray.find((value) => value > 0 && value % 2 === 0);

// result: 6
```

### reduce
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce): 
Executes a reducer function on each element of the array, accumulating a single result.

**Syntax**
```TypeScript
array.reduce(callback(accumulator, currentValue, currentIndex, array), initialValue)
```

Examples:

1. Sum all numbers in an array
```TypeScript
const originalArray = [1, 2, 3, 4];
const result = originalArray.reduce(
    (accumulator, currentValue) => accumulator + currentValue, 0);

// result: 10
```

2. Find the maximum value in an array
```TypeScript
const originalArray = [3, 7, 2, 9, 5];
const result = originalArray.reduce(
        (max, currentValue) => (currentValue > max ? currentValue : max), 
        -Infinity
    );

// result: 9
```

3. Flatten a nested array
```TypeScript
const originalArray = [[1, 2], [3, 4], [5]];
const result = originalArray.reduce(
        (flatArray, currentArray) => flatArray.concat(currentArray), []
    );

// result: [1, 2, 3, 4, 5]
```

---

### Java

## for Loop

```Java
int[] originalArray = {1, 2, 3, 4};
int[] newArray = new int[originalArray.length];

for (int i = 0; i < originalArray.length; i++) {
    newArray[i] = originalArray[i] * 2;
}

// newArray: [2, 4, 6, 8]
```
---

### Python
[🚧 To Be Added]

---

### Rust
[🚧 To Be Added]


