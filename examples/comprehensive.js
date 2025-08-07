// Comprehensive JavaScript parser test
// This file tests all supported features

// Variable declarations with different types
var number = 42;
var string = "Hello, World!";
var boolean = true;
var nullValue = null;
var undefinedValue = undefined;

// Arithmetic expressions
var sum = 10 + 5;
var difference = 20 - 8;
var product = 6 * 7;
var quotient = 15 / 3;
var remainder = 17 % 5;

// Comparison operators
var isEqual = (5 == 5);
var isNotEqual = (3 != 4);
var isStrictEqual = (7 === 7);
var isStrictNotEqual = (8 !== 9);
var isLess = (3 < 5);
var isLessOrEqual = (4 <= 4);
var isGreater = (6 > 2);
var isGreaterOrEqual = (7 >= 7);

// Logical operators
var logicalAnd = true && false;
var logicalOr = true || false;
var logicalNot = !true;

// Complex expressions with precedence
var complex = (2 + 3) * 4 - 1;
var condition = (x > 0) && (y < 100) || (z == 0);

// Function declarations
function add(a, b) {
    return a + b;
}

function factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

function greetUser(name) {
    var greeting = "Hello, " + name + "!";
    return greeting;
}

// Function calls
var result1 = add(10, 20);
var result2 = factorial(5);
var message = greetUser("Alice");

// Control flow statements
if (number > 40) {
    result1 = result1 * 2;
} else {
    result1 = 0;
}

// Loops
var counter = 0;
while (counter < 5) {
    counter = counter + 1;
}

var total = 0;
for (var i = 1; i <= 10; i = i + 1) {
    total = total + i;
}

// Arrays
var numbers = [1, 2, 3, 4, 5];
var mixed = [42, "text", true, null];
var nested = [[1, 2], [3, 4]];

// Objects
var person = {
    name: "Bob",
    age: 25,
    city: "New York"
};

var calculator = {
    add: function(x, y) {
        return x + y;
    },
    subtract: function(x, y) {
        return x - y;
    },
    multiply: function(x, y) {
        return x * y;
    }
};

// Property access
var personName = person.name;
var personAge = person.age;
var firstNumber = numbers[0];
var lastNumber = numbers[4];

// Method calls
var addResult = calculator.add(15, 25);
var subResult = calculator.subtract(50, 20);
var mulResult = calculator.multiply(3, 7);

// Nested object access
var nestedObject = {
    level1: {
        level2: {
            value: "deep value"
        }
    }
};

var deepValue = nestedObject.level1.level2.value;

// Assignment expressions
var x = 10;
x = x + 5;
var y = x;

// Block statements
if (true) {
    var blockVar = "inside block";
    {
        var nestedBlock = "nested";
    }
}

// Function with multiple parameters and complex logic
function processData(data, threshold, callback) {
    var processed = [];
    var count = 0;
    
    for (var i = 0; i < data.length; i = i + 1) {
        if (data[i] > threshold) {
            processed[count] = data[i] * 2;
            count = count + 1;
        }
    }
    
    return processed;
}
