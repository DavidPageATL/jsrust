// More complex JavaScript features
function factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

function fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Loop examples
var sum = 0;
for (var i = 1; i <= 10; i = i + 1) {
    sum = sum + i;
}

var count = 0;
while (count < 5) {
    count = count + 1;
}

// Object with methods
var calculator = {
    add: function(a, b) {
        return a + b;
    },
    multiply: function(a, b) {
        return a * b;
    }
};

var result1 = calculator.add(5, 3);
var result2 = calculator.multiply(4, 6);
