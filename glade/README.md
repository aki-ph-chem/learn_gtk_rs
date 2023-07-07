# Gladeを使う

これまでは、ウィジェットのインスタンスを生成するコードを書いてUIを構築していたが、これは`Glade`というGTK+のUIデザイナーを使うことでより直感的に作ることが可能である。

## Hello World

`Glade`を立ち上げて、左上の`Create a new project`をクリックする。
次に、上のダイアログのうちの`Toplevel`から`GtkWindow`を選びクリックする。
こうして`GtkWindow`が生成・選択される。
次に、右側の画面からの`ID`を"window\_1"にして、`Title`を"Hello World"にする。

こうして生成されたUIのファイルを`src/ui/hello.ui`として保存する。
続いてソースコードを書いていく。uiのファイルから`gtk::window`のインスタンスを生成するコードは以下のようになる。

```Rust
let ui = include_str!("ui/hello.ui");
let builder = gtk::Builder::from_string(ui);

let window: gtk::Window = builder.object("window_1").expect("Error: window_1");
```

このコードでは、まずuiファイルを読み取り、文字列に変換する。
続いて、この文字列を`gtk::Builder::from_string()`に渡し、`Builder`を生成する。
このBuilderを用いて`gtk::Window`のインスタンスを生成する。

この処理が終われば以降はコードで記述していた場合と同じように開発を進める。
