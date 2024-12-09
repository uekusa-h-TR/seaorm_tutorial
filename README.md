# 検証項目

- entityのインポートの仕方
  - workspace機能でentityをインポートしているのはわかったが、大文字のentityと小文字のentityで何が違うのか？

## チュートリアルから変えたこと

- `sea-orm-cli generate eneity -o entity/src`で生成したエンティティファイルのentity/src/mod.rsをentity/src/lib.rsに変えた。