{
    const originalArray = [1, 2, 3];
    const newArray = [];
    for (let i = 0; i < originalArray.length; i++) {
        newArray.push(originalArray[i] * 2);
    }
    console.log({ name: 'for', originalArray, newArray });
};

{
    const originalArray = [1, 2, 3];
    const newArray = [];
    for (const value of originalArray) {
        newArray.push(value * 2);
    }
    console.log({ name: 'for...of', originalArray, newArray });
};

{
    const originalArray = [1, 2, 3];
    const newArray = [];
    let i = 0;
    while (i < originalArray.length) {
        newArray.push(originalArray[i] * 2);
        i++;
    }
    console.log({ name: 'while', originalArray, newArray });
};

{
    const originalArray = [1, 2, 3];
    const newArray = [];
    let i = 0;
    do {
        newArray.push(originalArray[i] * 2);
        i++;
    } while (i < originalArray.length);
    console.log({ name: 'do...while', originalArray, newArray });
};

{
    const originalArray = [1, 2, 3, 4];
    const newArray = [];
    originalArray.forEach((value) => {
        newArray.push(value * 2);
    });
    console.log({ name: 'forEach', originalArray, newArray });
};

{
    const originalArray = [1, 2, 3, 4];
    const newArray = originalArray.map((value) => value * 2);
    console.log({ name: 'map', originalArray, newArray });
};

{
    const originalArray = [1, -2, 3, 4, -5];
    const result = originalArray.filter((value) => value > 0 );
    console.log({ name: 'filter', originalArray, result });
};

{
    const originalArray = [-1, 3, 6, 7, 8];
    const result = originalArray.find((value) => {
        return value > 0 && value % 2 === 0;
    });
    console.log({ name: 'find', originalArray, result });
};

{
    const originalArray = [2, 4, 6];
    const result = originalArray.every((value) => value % 2 === 0);
    console.log({ name: 'every', originalArray, result });
};

{
    const originalArray = [1, 3, 5, 6];
    const result = originalArray.some((value) => value % 2 === 0);
    console.log({ name: 'some', originalArray, result });
};

{
    const originalArray = [1, 2, 3, 4];
    const result = originalArray.reduce(
        (accumulator, currentValue) => accumulator + currentValue, 0
    );
    console.log({ name: 'reduce...sumArray', originalArray, result });
};

{
    const originalArray = [3, 7, 2, 9, 5];
    const result = originalArray.reduce(
        (max, currentValue) => (currentValue > max ? currentValue : max), 
        -Infinity
    );
    console.log({ name: 'reduce...maxVal', originalArray, result });
};

{
    const originalArray = [[1, 2], [3, 4], [5]];
    const result = originalArray.reduce(
        (flatArray, currentArray) => flatArray.concat(currentArray), []
    );
    console.log({ name: 'reduce...flattenArray', originalArray, result });
};