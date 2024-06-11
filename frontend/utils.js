export function movingAverage(data, windowSize) {
  let result = [];
  for (let i = 0; i < data.length; i++) {
    let start = Math.max(i - windowSize + 1, 0);
    let avg = data.slice(start, i + 1).reduce((a, b) => a + b, 0) / windowSize;
    result.push(avg);
  }
  return result;
}
