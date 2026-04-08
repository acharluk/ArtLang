export const examples = [
  {
    id: 'hello',
    label: 'Hello World',
    code: `println("Hello, ArtLang!")`,
  },
  {
    id: 'arithmetic',
    label: 'Arithmetic',
    code: `a = 10
b = 3

println("a + b = " .. (a + b))
println("a - b = " .. (a - b))
println("a * b = " .. (a * b))
println("a / b = " .. (a / b))
println("a % b = " .. (a % b))
println("a ^ b = " .. (a ^ b))`,
  },
  {
    id: 'factorial',
    label: 'Factorial',
    code: `function factorial(n)
    if n <= 1 then
        return 1
    end
    return n * factorial(n - 1)
end

for i = 1, 12 do
    println("factorial(" .. i .. ") = " .. factorial(i))
end`,
  },
  {
    id: 'fibonacci',
    label: 'Fibonacci',
    code: `function fib(n)
    if n <= 1 then
        return n
    end
    return fib(n - 1) + fib(n - 2)
end

for i = 0, 20 do
    println("fib(" .. i .. ") = " .. fib(i))
end`,
  },
  {
    id: 'closures',
    label: 'Closures',
    code: `-- Higher-order functions and closures
fn adder(x)
    return fn(y) return x + y end
end

add5 = adder(5)
add10 = adder(10)

println("add5(3) = " .. add5(3))
println("add10(7) = " .. add10(7))

-- Function that takes a function
fn apply(f, x)
    return f(x)
end

println("apply(add5, 20) = " .. apply(add5, 20))`,
  },
  {
    id: 'fizzbuzz',
    label: 'FizzBuzz',
    code: `for i = 1, 30 do
    if i % 15 == 0 then
        println("FizzBuzz")
    elseif i % 3 == 0 then
        println("Fizz")
    elseif i % 5 == 0 then
        println("Buzz")
    else
        println(i)
    end
end`,
  },
]
