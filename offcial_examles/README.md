# 公式の情報を読んで

公式のチュートリアルを読んで内容を日本語でまとめる。

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
