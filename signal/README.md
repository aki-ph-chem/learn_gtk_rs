# シグナルとイベント

## Windowを閉じる

現段階では、windowを閉じてもプログラムは終了しない。ここではwindowを閉じるイベントに対して、イベントハンドラを実装してプログラムを終了するようにする。

メソッド`connect_delete_event()`に対して、イベントが起こったときの動作をクロージャで与える。

```Rust
window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(true)
        });
```

クロージャに渡す変数には`Self`,`Event`があるが、ここではどちらも不要であるので、`_`で無視している。また、返り値の`Inhibit`はデフォルトのハンドラを実行するか否かである。ここではどちらでも良い。
最後に全体を示す。


```Rust
extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello Wrold");
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();

    Ok(())
}
```

## ウィジェットの値をハンドラで利用する 

次は一行のテキスト入力`gtk::Entry`を追加して、ボタンが押されたらコンソールに内容が表示されるようにする。

Buttonが押されたイベントをハンドルするには`connect_clicked()`を用いる。

```Rust
let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

let entry = gtk::Entry::new();
hbox.pack_start(&entry, true, true, 5);

let button = gtk::Button::with_label("Say");
button.connect_clicked(move |_| {
        println!("Text: {}", entry.text());
        });
```

ここで、重要なのは、`connect_clicked()`に渡すクロージャに`move`を付けてbuttonの所有権をクロージャに譲渡している点である。
これは、entryよりもクロージャの寿命が長くなる可能性があるためである。

最後に全体のソースコードを示す。


```Rust
extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello Wrold");
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let entry = gtk::Entry::new();
    hbox.pack_start(&entry, true, true, 5);
    
    let button = gtk::Button::with_label("Say");
    button.connect_clicked(move |_| {
        println!("Text: {}", entry.text());
    });

    hbox.pack_start(&button, false, false, 5);

    window.add(&hbox);
    window.show_all();
    gtk::main();

    Ok(())
}
```

## メッセージダイアログ 

今度は得た文字列をprintln!()でコンソールに出力するのではなく、メッセージダイアログに表示してみる。

先程と同じように`connect_clicked()`でbuttonとイベントを関連付ける。

重要なのは、いちどwindowのクローンを作成している点である。
これは、`gtk::MessageDialog`を生成するためにToplevelのwindowへの参照をOPtionでラップした値が必要であるが、`connect_cliked()`に与えるクロージャに与えてしまうと、その後使えなくなってしまうためである。
ただ、この`clone()`はGTK+で利用しているGObjectのリファレンスカウントを増加させるものであってdeep copyされるわけではない。

```Rust
let window_ = window.clone(); 
button.connect_clicked(move |_| {
        let message = format!("Text: {}", entry.text());

        let dialog = gtk::MessageDialog::new(
                Some(&window_),
                gtk::DialogFlags::empty(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                &message);

        dialog.run();
        unsafe {
        dialog.destroy();
        }
        });
```

最後に全体のソースコードを示す
```Rust
extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello World");
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(true)
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let entry = gtk::Entry::new();
    hbox.pack_start(&entry, true, true, 5);

    let button = gtk::Button::with_label("Say");

    let window_ = window.clone(); 
    button.connect_clicked(move |_| {
        let message = format!("Text: {}", entry.text());

        let dialog = gtk::MessageDialog::new(
            Some(&window_),
            gtk::DialogFlags::empty(),
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            &message);

        dialog.run();
        unsafe {
            dialog.destroy();
        }
    });

    hbox.pack_start(&button, false, false, 5);

    window.add(&hbox);
    window.show_all();
    gtk::main();

    Ok(())
}
```
