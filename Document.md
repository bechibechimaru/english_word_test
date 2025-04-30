# 英単語出力プロジェクト

## 関数

- generate_random_number
  - start_number, end_number
  - number


## crate

使用したクレート


クレートの選定

### rand vs random_number


wordlist = 一括取得する
wordlistから指定のindexの英単語を取得する

クエリを工夫する。
SELECT english_word WHERE index in (number1, number2...numnberi)

1. 
`SELECT * FROM user`

実行時間：9.370667ms

2. 
`SELECT * FROM user WHERE id IN (?,?,....?)`

実行時間：4.649416ms

3. 
`
  for i in 1..=50{
      SELECT * FROM USER WHERE id = ?
      .bind(i)
  }
`

実行時間：64.888ms
