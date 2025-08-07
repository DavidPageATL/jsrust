// Quick test of all major features
var x = 10;
var msg = "test";

function test(a) {
    return a + 1;
}

if (x > 5) {
    x = test(x);
}

var arr = [1, 2, 3];
var obj = {name: "test", value: 42};
var result = obj.value + arr[0];
