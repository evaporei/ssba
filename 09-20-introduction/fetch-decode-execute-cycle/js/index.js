const start = new Date().getTime();
for (let i = 0; i < 1000000000; i++);
const diff = new Date().getTime() - start
console.log('elapsed:', diff)
const secs = diff / 1000
const ops = 1000000000 / secs
console.log('clock speed approx: ', ops / 1000000000)
