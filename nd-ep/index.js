const express = require('express');
const app = express();

app.use(express.json({ limit: '30mb' }));

app.use(express.urlencoded({ limit: '30mb', extended: true }));


function countPrimes(limit) {
    let count = 0;

    for (let num = 2; num <= limit; num++) {
        let isPrime = true;
        const maxDivisor = Math.sqrt(num);

        for (let i = 2; i <= maxDivisor; i++) {
            if (num % i === 0) {
                isPrime = false;
                break;
            }
        }

        if (isPrime) {
            count++;
        }
    }

    return count;
}

app.get('/ping', (req, res) => {
    const a = countPrimes(1000000);

    res.send(`Hello World! ${a}`);

});

app.post('/parsetest', (req, res) => {
    const body = req.body;
    res.send(JSON.parse(body));
});

function factorize(n) {
    const factors = [];
    let d = 2;
    while (n > 1) {
        while (n % d === 0) {
            factors.push(d);
            n /= d;
        }
        d += 1;
        if (d * d > n) {
            if (n > 1) {
                factors.push(n);
            }
            break;
        }
    }
    return factors;
}

app.get('/factorize', (req, res) => {
    const startTime = Date.now();

    for (let i = 0; i < 100; i++) {
        factorize(100000000000031);
    }

    const duration = Date.now() - startTime;
    res.send(`Completed in: ${duration} ms`);
});


app.get('/mat', (req, res) => {
    const N = 5000;
    let a = Array(N).fill().map(() => Array(N).fill(1.0));
    let b = Array(N).fill().map(() => Array(N).fill(2.0));
    let c = Array(N).fill().map(() => Array(N).fill(0.0));

    // Start timing the matrix multiplication
    console.time('Matrix Multiplication');

    for (let i = 0; i < N; i++) {
        for (let j = 0; j < N; j++) {
            for (let k = 0; k < N; k++) {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    console.timeEnd('Matrix Multiplication');

    // Send response
    res.send(`Matrix multiplication completed. Result: ${c[0][0]} `);
});

const PORT = 3000;
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
