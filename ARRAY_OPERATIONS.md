# Array Operations

Demonstrating a variety of array-related operations that can be performed across multiple programming languages. 

Covering different techniques for iterating, transforming, and manipulating arrays.

---

## Languages Covered
  - [JavaScript/TypeScript](#javascripttypescript)
      - [for](#for-loop), [for...of](#forof-loop), [while](#while-loop), [do...while](#dowhile-loop)
      - [map](#map), [forEach](#foreach), [filter](#filter), [some](#some), [every](#every), [find](#find)
  - [Java](#java)
  - [Python](#python)
  - [Rust](#rust)

# JavaScript/TypeScript
## JavaScript/TypeScript Constructs

### For Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for): 
A traditional loop using an index or counter.

```TypeScript
/**
 * Iterates over an array of numbers and returns a new array with each element doubled.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number[]} A new array where each element is twice the corresponding element in the input array.
 */
const forLoop = (originalArray: number[]) => {
    const newArray = [];
    for (let i = 0; i < originalArray.length; i++) {
        newArray.push(originalArray[i] * 2);
    }
    return newArray; 
};
```

### for...of Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of): 
Iterates over the values of an iterable (e.g., arrays, strings).

```TypeScript
/**
 * Iterates over an array of numbers and returns a new array with each element doubled.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number[]} A new array where each element is twice the corresponding element in the input array.
 */
const forOfLoop = (originalArray: number[]) => {
    const newArray = [];
    for (const value of originalArray) {
        newArray.push(value * 2);
    }
    return newArray;
};
```

### while Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/while): 
Executes a block of code as long as the condition is `true`.

```TypeScript
/**
 * Demonstrates a while loop by doubling each value in the original array.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number[]} A new array containing elements of the original array multiplied by 2.
 */
const whileLoop = (originalArray: number[]) => {
    const newArray = [];
    let i = 0;

    while (i < originalArray.length) {
        newArray.push(originalArray[i] * 2);
        i++;
    }

    return newArray;
};
```

### do...while Loop
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/do...while):
Similar to `while`, but the code block runs at least once. Runs at least once even if the `do` condition is `false`.

```TypeScript
/**
 * Demonstrates a do-while loop by doubling each value in the original array.
 * Runs at least once.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number[]} A new array containing elements of the original array multiplied by 2.
 */
const doWhileLoop = (originalArray: number[]) => {
    const newArray = [];
    let i = 0;

    do {
        newArray.push(originalArray[i] * 2);
        i++;
    } while (i < originalArray.length);

    return newArray;
};
```

## JavaScript/TypeScript Array Methods

### forEach
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach): 
Iterates over an array, executing a callback for each element.

```TypeScript
/**
 * Demonstrates the use of the forEach array method by doubling each value in the original array.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number[]} A new array containing elements of the original array multiplied by 2.
 */
const forEachLoop = (originalArray: number[]) => {
    const newArray = [];

    originalArray.forEach((value) => { newArray.push(value * 2); });

    return newArray;
};
```

### map
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map): 
Creates a new array by applying a function to each element.

```TypeScript
/**
 * Demonstrates the use of the map array method by doubling each value in the original array.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number[]} A new array containing elements of the original array multiplied by 2.
 */
const mapMethod = (originalArray: number[]) => {
  return originalArray.map((value) => value * 2);
}
```

### filter
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter): 
Creates a new array with elements that pass a test.

```TypeScript
/**
 * Demonstrates the use of the filter array method to find even positive numbers.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number[]} A new array containing only even positive numbers from the original array.
 */
const filterArray = (originalArray: number[]) => {
    return originalArray.filter((value) => value > 0 && value % 2 === 0);
};
```

### every
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every):
Checks if all elements in the array satisfy the provided condition and returns `true` only if they all do; otherwise, returns `false`.

```TypeScript
/**
 * Checks if every item in the array is a positive even number.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {boolean} `true` if all items in the array are positive even numbers, otherwise `false`.
 */
const everyItem = (originalArray: number[]) => {
    return originalArray.every((item) => item > 0 && item % 2 === 0);
}
```

### some
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some): 
Checks if at least one element in the array satisfies the provided condition and returns `true` if it does; otherwise, returns `false`.

```TypeScript
/**
 * Checks if at least one item in the array is a positive even number.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {boolean} `true` if the array contains at least one positive even number, otherwise `false`.
 */
const someItem = (originalArray: number[]) => {
    return originalArray.some((item) => item > 0 && item % 2 === 0);
}
```

### find
[Documentation 🔗](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find): 
Returns the first element that satisfies a condition.

```TypeScript
/**
 * Demonstrates the use of the find array method to locate the first number meeting multiple conditions.
 * @param {number[]} originalArray - The input array of numbers.
 * @returns {number|undefined} The first number greater than 0, divisible by 3, and divisible by 2; or undefined if not found.
 */
const findArrayItem = (originalArray: number[]) => {
    return originalArray.find((value) => (value > 0
        && value % 3 === 0 && value % 2 === 0)
    );
};
```

---

### Java
[🚧 To Be Added]

---

### Python
[🚧 To Be Added]

---

### Rust
[🚧 To Be Added]


