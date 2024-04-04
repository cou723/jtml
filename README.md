# alt-html(jtml)
プログラミング言語の小規模エコシステムを作ってみるという試み
## 目標
- jtmlからhtmlへのトランスパイラ作成
- syntax highlightをつけられるようにvscodeの拡張作成
- formatterの作成

## 発展
- 構文解析してエラーを表示するvscode拡張機能の作成

## 処理の流れ

jtml =(jtmlLexer)=> tokens =(parser)=> ast =(transformer)=> ast =(code generator)=> html

## 用語集
- jtml: 当プロジェクト、またはその言語
- Document: 一つのファイルにつき一つ。複数のAST Nodeを持つ
- AstNode: ElementまたはStringLiteral, Commentを指す
- Element: HTMLのエレメント(`<p>hoge</p>`)やJTMLのエレメント(`p(){hoge}`)を指す
- StringLiteral: 文字列リテラルを指す。`<p>hoge</p>`の`hoge`の部分
- attributes: Elementについている属性。htmlの`<p class="hoge">`やjtmlの`p(class="hoge"){hoge}`の`class="hoge"`の部分