# React(frontend)のコーディングスタイル

## 概要

- UIの表示にのみに責務を持つpresentation層と、実際のボタンクリック時の挙動やリクエスト処理等を記述するcontainer層に分割する。
- presentation層は`frontend/src/presentation`配下に実装する。
- container層は`frontend/src/container`配下に実装する。

このファイルは適宜更新されるが、現在記述されているスタイルが最新であることとする。

## presentation層

presentation層はUIにのみ責務を持つため、実際の処理等はpropsでcontainer層から注入することを想定する。

presentation層はページのUIを担当するpagesと共通のコンポーネントを表すcomponentsのディレクトリ、そして入力に対するvalidationを行うvalidationsディレクトリを持つ。想定されるディレクトリ構造は下記の通り。

```
.
└── frontend
    ├── presentation
    │   ├── pages
    │   ├── components
    │   └── validations
    └── container
```

pagesはページ名ごとに、componentsはコンポーネント名ごとにディレクトリが存在する。

## container層

container層は基本的にpresentation/pagesに存在するpageごとに、対応するファイルを作成する。

container層で行う処理について、共通処理が存在すればhelperディレクトリにファイルを作成し、そこに共通処理を記述する。
想定されるディレクトリ構造は下記の通り。

```
.
└── frontend
    ├── presentation
    └── container
        ├── pages
        └── helper
```


