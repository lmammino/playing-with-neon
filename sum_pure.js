const numbers = require('./numbers')

const total = numbers.reduce((acc, n) => acc + (n*n), 0)
console.log(total)
