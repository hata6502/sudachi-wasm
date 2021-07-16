console.log("loading...");

const { TokenizeMode, tokenize } = await import("../pkg/sudachi.js");

console.log("tokenizing 1...");
console.log(JSON.parse(tokenize("今日は良い天気なり。", TokenizeMode.C)));

console.log("tokenizing 2...");
console.log(JSON.parse(tokenize("明日は悪い予報なり。", TokenizeMode.B)));

console.log("tokenizing 3... (It will fail)");
console.log(
  JSON.parse(
    tokenize(
      `
        親譲りの無鉄砲で小供の時から損ばかりしている。
        小学校に居る時分学校の二階から飛び降りて一週間ほど腰を抜かした事がある。
        なぜそんな無闇をしたと聞く人があるかも知れぬ。
        別段深い理由でもない。
        新築の二階から首を出していたら、同級生の一人が冗談に、いくら威張っても、そこから飛び降りる事は出来まい。
        弱虫やーい。
        と囃したからである。
        小使に負ぶさって帰って来た時、おやじが大きな眼をして二階ぐらいから飛び降りて腰を抜かす奴があるかと云ったから、この次は抜かさずに飛んで見せますと答えた。
        （青空文庫より）
      `,
      TokenizeMode.A
    )
  )
);
