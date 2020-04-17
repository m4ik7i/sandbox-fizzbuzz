const main = (): void => {
  const n = Number(Deno.args[0]);

  Array.from(Array(n), (_v, k) => k + 1).forEach((i) => {
    console.log(fizzbuzz(i));
  });
};

const fizzbuzz = (n: number): string => {
  if (n % 3 == 0 && n % 5 == 0) {
    return "FizzBuzz";
  } else if (n % 3 == 0) {
    return "Fizz";
  } else if (n % 5 == 0) {
    return "Buzz";
  } else {
    return `${n}`;
  }
};

if (import.meta.main) {
  main();
}
