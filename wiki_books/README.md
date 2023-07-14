# Wiki Books,ZetCode の GTK+のC言語のサンプルをRustで

## Contents

このディレクトリである`wiki_books`に含まれるソースコードの説明。
あるコードが`<src_name>.rs`であった場合、実行するには

```bash
$ cargo run --bin <src_name>
```

を実行すれば良い。
またGladeによって生成された、UIを構成しているXMLファイルは`ui`ディレクトリ下に存在する。

1. gtk\_1.rs: ボタン,labelの実装されたwindowを表示するだけのプログラム
2. gtk\_1f.rs: gtk\_1.rsのUI構築処理をbuilderで実装したもの 
3. gtk\_1fq.rs: gtk\_1.rsにボタンのクリックor Ctrl + oで終了する処理を実装したもの
4. gtk\_2.rs: main()からlabelの内容を変更する
5. gtk\_2f.rs: gtk\_2.rsのUI構築処理をbuilderで実装したもの
5. gtk\_3.rs: メニュバーを実装し、Help/Aboutのクリックで、Aboutダイアログが表示され、File/Quitのクリックor Ctrl + qで終了する 
5. gtk\_3f.rs: gtk\_3.rsのUI構築処理をbuilderで実装したもの

## Windowのサイズ

Gladeで`GtkWindow`が選択された状態で、右側の画面で`Appearance`の`Default Width`,`Default Height`の数値を設定すれば良い。

## Labelの設定

Gladeで`GtkLabel`が選択された状態で、右側の画面で`Alignment and Padding`の`Alignment`でLabelの位置を調節する。

## Widgetの初期化の処理

二通りの方法がある

1. main()内で `gtk::init()`してから初期化処理を行い`gtk::main()`でイベントループに入る。 

2. `gtk::Application`のインスタンスを生成して、`connect_activate()`メソッドに初期化を行う関数を渡して、初期化処理を行い`run()`メソッドでイベントループに入る。

## ショートカットkey

`gtk::AccelGroup`を使う。`gtk::AccelGroup::new()`でインスタンスを生成して、Toplevel Widgetに`add_accel_group()`メソッドで加える。
`gtk::accelerator_parse()`でkeyの入力をパースして、その返り値(タプルを返す)と他のいくつかのパラメータをボタン等のオブジェクトに対して`add_accelerator()`メソッドで加える。

`add_accel_group()`のシグニチャ
```Rust
fn add_accel_group(&self, &gtk::AccelGroup)
```

`acccelerator_parse()`のシグニチャ
```Rust
pub fn accelerator_parse(accelerator: &str) -> (u32, ModifierType)
```

`add_accelerator()`のシグニチャ
```Rust
fn add_accelerator(
    &self,
    accel_signal: &str,
    accel_group: &impl IsA<AccelGroup>,
    accel_key: u32,
    accel_mods: ModifierType,
    accel_flags: AccelFlags
)
```

例:
```Rust
// File/Quitをクリックでプログラムを終了
let quit: gtk::MenuItem = builder.object("quit")
.expect("Error: quit");
let window_ = window.clone();
quit.connect_activate(move |_|{
        window_.close();
        });

// Ctrl + qで終了
let accel_group = gtk::AccelGroup::new(); 
window.add_accel_group(&accel_group);
let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
quit.add_accelerator("activate", &accel_group, key, 
        modifier, gtk::AccelFlags::VISIBLE);
```

## 参考

- wikibooks: https://ja.wikibooks.org/wiki/GTK%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0
- ZetCode: https://zetcode.com/gui/gtk2/ 
- gtk3-rsの公式リポジトリ: https://github.com/gtk-rs/gtk3-rs/tree/master/examples 
