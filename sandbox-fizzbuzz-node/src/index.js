const fizzbuzz = (n) => {
  if (n % 3 == 0 && n % 5 != 0) {
    return 'Fizz';
  } else if (n % 5 == 0 && n % 3 != 0) {
    return 'Buzz';
  } else if (n % 3 == 0 && n % 5 == 0) {
    return 'FizzBuzz';
  } else {
    return `${n}`;
  }
};

const main = () => {
  const n = Number(process.argv[2]);

  Array.from(Array(n), (v, k) => k + 1).forEach((i) => {
    console.log(fizzbuzz(i));
  });
};

main();
