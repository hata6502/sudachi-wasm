console.log('loading...');

const { tokenize } = await import('../pkg/sudachi.js');

console.log('tokenizing 1...');
console.log(JSON.parse(tokenize('今日は良い天気なり。')));
console.log('tokenizing 2...');
console.log(JSON.parse(tokenize('明日は悪い予報なり。')));
