# 公式の情報を読んで

公式のチュートリアルを読んで内容を日本語でまとめる。

## Contents

このディレクトリである`wiki_books`に含まれるソースコードの説明。
あるコードが`<src_name>.rs`であった場合、実行するには

```bash
$ cargo run --bin <src_name>
```

を実行すれば良い。
またGladeによって生成された、UIを構成しているXMLファイルは`ui`ディレクトリ下に存在する。

1. button\_inc.rs: ボタンをクリックして表示される数値を変化させる。
2. file\_chose.rs: テキストファイルを選択して内容をwindow内に表示する。
3. file\_chose\_b.rs: テキストファイルを選択して内容をwindow内に表示する。ただし、ボタンの実装はGlade側ではなくソースコード側
4. grid.rs: window内にgridを置き、その上にボタンを配置し、イベントを実装した。 
5. use\_clone.rs: clone()!マクロを使う。実装した内容はbutton\_inc.rsと同じ。
6. refcell\_rc.rs: スマートポインタ Rc,RefCellを使う。
7. clock.rs: window上に時間を表示する。

## Callbackとクロージャー 

### コールバックとクロージャー

GUIライブラリではあるイベントが発生した時の挙動を、コールバック関数で実装することは一般的である。
`Gtk-rs`ではクロージャで実装することができる。関数ポインタよりも便利であるがライフタイムを追いかけるのは難しい。
`Gtk-rs`では全てのオブジェクトに対してクロージャが呼び出されてときに、クロージャによって取り込まれたオブジェクトが"まだ生きている"ことを保証するために、
lifetimeが`static`であることを要求する。

Cでの実装は

```C
#include <gtk/gtk.h>

void callback_clicked(GtkWidget *widget, gpointer data) {
    gtk_button_set_label(GTK_BUTTON(widget), "Window");
}

GtkWidget *button = gtk_button_new_with_label("Click me!");
g_signal_connect(button, "clicked", G_CALLBACK(callback_clicked), NULL);
```

Rustではコールバックではなく、`connect_clicked()`メソッドにクロージャを渡すことでイベント(ここではボタンのクリック)が起こったときの処理を行う。

```Rust
use gtk::{Button, ButtonExt};

let button = Button::new_with_label("Click me!");
button.connect_clicked(|but| {
    but.set_label("I've been clicked!");
});
```

### clone()

シンプルな例としては以下を考える。

```Rust
use gtk::{Box, Button, ButtonExt, ContainerExt, WidgetExt};

// First we create a layout.
let container = Box::new(gtk::Orientation::Vertical, 5);
// the label which will be modified inside the closure.
let label = gtk::Label::new("");
let button = Button::new_with_label("Click me!");
button.connect_clicked(move |_| {
    label.set_label("Button has been clicked!");
});

container.add(&button);
container.add(&label);
```

しかし、このコードはコンパイルできず、以下のエラーメッセージを得る

```text
error[E0382]: use of moved value: `label`
```

これは、クロージャの前に`move`が付いており、クロージャ内でアクセスしたオブジェクトはクロージャに所有権が移ってしまっているためである。
これを動くようにするためには以下のように`label`を一旦`clone()`によってコピーを作って、それを`connect_clicked`に渡すようにのように修正する。

```Rust
use gtk::{Box, Button, ButtonExt, ContainerExt, WidgetExt};

// First we create a layout.
let container = Box::new(gtk::Orientation::Vertical, 5);
// the label which will be modified inside the closure.
let label = gtk::Label::new("");
let button = Button::new_with_label("Click me!");
// We clone label so we can send it into the closure.
let label_clone = label.clone();
button.connect_clicked(move |_| {
    label_clone.set_label("Button has been clicked!");
});

container.add(&button);
container.add(&label);
```

ここで、`clone()`メソッドを使うと大きなコストが掛かるように思うかもしれない。
しかし、`Gtk-rs`のオブジェクトに対して`clone()`メソッドでコピーするのにはポインタをコピーするコストしかかからないため心配は要らない。

## `Gtk-rs`オブジェクトとClosure

もう少し複雑な例を考えてみよう。マルチwindowなプログラムで、windowを追跡して複数のクロージャーからアクセスできるようにしたいとする。

これを実現する一つの方法は標準ライブラリに含まれるスマートポインタである`Rc`,`RefCell`を使う方法である。
以下に例を示す。

```Rust
use gtk::{Button, ButtonExt, Window};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

let windows: Rc<RefCell<HashMap<usize, Window>>> = Rc::new(RefCell::new(HashMap::new()));
let button = Button::new_with_label("Click me!");

// We copy the reference to the cell containing the hashmap.
let windows_clone = windows.clone();
button.connect_clicked(move |_| {
    // create_window functions creates a window and return the following tuple: (usize, Window).
    let (window_id, window) = create_window();
    windows_clone.borrow_mut().unwrap().insert(window_id, window);
});

 ...

another_button.connect_clicked(move |_| {
    let id_to_remove = get_id_to_remove();
    windows.borrow_mut().unwrap().remove(&id_to_remove);
});
```

ある型`T`を`Rc<RefCell<T>>`でラップしているのは、可変で複数の所有者を持てるようにするためである。

しかし、マクロ`clone()!`を使うとより容易に実装することができる。

```Rust
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}
```

このマクロを使うと上のコードは以下のように書ける。


```Rust
let windows: Rc<RefCell<HashMap<usize, Window>>> = Rc::new(RefCell::new(HashMap::new()));
button.connect_clicked(clone!(windows => move |_| {
    let (window_id, window) = create_window();
    windows.borrow_mut().unwrap().insert(window_id, window);
}));
```

### 参考
- Callbacks and closures: https://martinber.github.io/gtk-rs.github.io/docs-src/tutorial/closures 

##  Upcastとdowncast

### Upcasting

Gtkには継承システムが存在するため、`Gtk-rs`にも当然存在する。多くの人々は通常必要としないであろうが、仕組みを理解しておくことは重要である。

Upcastingはシンプルで以下のように書く

```Rust
let button = gtk::Button::new_with_label("Click me!");
let widget = button.upcast::<gtk::Widget>();
```

ここで、構造体`Button`はトレイト`IsA<Widget>`を実装しているため、構造体`Widget`にキャストすること(upcast)が可能である。
ここで、トレイト`IsA`は全てのwidgetとその親オブジェクトに実装されている、そのためここでは`Button`から`Widget`へのupcastが可能となった。

## widgetが別のwidgetかどうかのチェック

チェックの例として、`Widget`が`Box`であるかを判定する例を考える。
その場合、以下のようなジェネリックな関数を考えればよい。

```Rust
fn is_a_box<W: IsA<gtk::Object> + IsA<gtk::Widget> + Clone>(widget: &W) -> bool {
    widget.clone().upcast::<gtk::Widget>().is::<gtk::Box>()
}
```

この関数が何をしているかを考えてみよう。
引数のwidgetはトレイト境界にもある通り、`IsA<gtk::Widget>`,`IsA<gtk::Object>`を実装していることが要求される。
また、`gtk::Object`には`Cast`トレイトが実装されていてメソッド`upcast()`,`downcast`が呼び出し可能であることも要求する。

実際では、widgetに対して`IsA<gtk::Widget>`をトレイト境界に要求することは必要ではないが、問題を単純化するために(`gtk::Widget`への`upcast`がトラブルなく行えることを目的に)要求している。

関数の実装のポイントはwidgetをより高階層のオブジェクトにupcastして、判定したオブジェクトにdowncastしている点である。

よりジェネリックにしたバージョンは以下のように実装できる。

```Rust
fn is_a<W: IsA<gtk::Object> + IsA<gtk::Widget> + Clone,
        T: IsA<gtk::Object> + IsA<gtk::Widget>>(widget: &W) -> bool {
    widget.clone().upcast::<gtk::Widget>().downcast::<T>().is_ok()
}
```

この関数は次のテストをpassする。


```Rust
#[test]
fn test_is_a() {
    let button = gtk::Button::new_with_label("Click me!");

    assert_eq!(is_a::<_, gtk::Container>(&button), true);
    assert_eq!(is_a::<_, gtk::Label>(&button), false);
}

```

### 参考
- Upcast and downcast: https://martinber.github.io/gtk-rs.github.io/docs-src/tutorial/upcast_downcast 

## Gladeの利用

### about Glade

Gladeは容易に`Gtk`アプリケーションのUIを開発を可能にするツールである。
設計されたUIはxmlファイルとして出力され、それを読み込んでUIの構築を行う。

### 例

Gladeを使って`gtk::Window`,`Gtk::Button`を含むUIを設計したとする。また、メッセージを含んでいる`gtk::MessageDialog`と`gtk::Label`も含まれているとする。

生成されたxmlファイルが`basic.ui`であったとする。このときこのファイルを読込み、
UIを構築する起点となる`gtk::Builder`のインスタンスを生成するには以下のコードを書けば良い。

```Rust
// First we get the file content.
let glade_src = include_str!("builder_basics.glade");
// Then we call the Builder call.
let builder = gtk::Builder::new_from_string(glade_src);
```

ここで`indluce_str!()`マクロはコンパイル時にファイルを読込みString型を返す。

builderから`gtk::Window`のインスタンスを生成するためには`get_object()`メソッドを用いる、引数にはGladeでUIを設計したときに割り振ったIDを&str型としてを与える

```Rust
// Our window id is "window1".
let window: gtk::Window = builder.get_object("window1").unwrap();
window.show_all();
```

次にシグナルのハンドリングについて考えよう。
ここではボタン(`gtk::Button`)をクリックするイベントによってメッセージダイアログ(`gtk::MessageDialog`)が表示される例を考える。
ボタンのクリックのイベントを拾えばいいので、`connect_clicked()`メソッドを用いて以下のように実装する。

```Rust
let button: gtk::Button = builder.get_object("button1").unwrap();
let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

button.connect_clicked(move |_| {
    // We make the dialog window blocks all other windows.
    dialog.run();
    // When it finished running, we hide it again.
    dialog.hide();
});
```

最後にコード全体を示す。

```Rust
fn main() {
    gtk::init().expect("Failed to initilize GTK"); 

    let glade_src = include_str!("builder_basics.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").unwrap();
    let button: gtk::Button = builder.get_object("button1").unwrap();
    let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

    button.connect_clicked(move |_| {
            dialog.run();
            dialog.hide();
            });

    window.show_all();

    gtk::main();
}
```


### 参考
- Glade: https://martinber.github.io/gtk-rs.github.io/docs-src/tutorial/glade
