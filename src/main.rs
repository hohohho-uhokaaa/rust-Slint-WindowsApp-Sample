// Windows 11 の GUI アプリとして動作させるため、起動時にコンソールウィンドウを表示させない
#![windows_subsystem = "windows"]

// Slint の UI 定義をインラインで記述し、Rust コードに組み込むマクロ
slint::slint! {
    // 標準ウィジェット（Button / Text / VerticalLayout など）をインポート
    import { Button } from "std-widgets.slint";

    // メインウィンドウコンポーネント（export により Rust 側から MyApp として利用可能）
    export component MyApp inherits Window {
        // ウィンドウの推奨サイズを指定
        preferred-width: 420px;
        preferred-height: 220px;
        // ウィンドウのタイトルを指定
        title: "Slint Demo";

        // 画面に表示するメッセージ（ボタンクリックで更新される in-out プロパティ）
        // ※コンポーネント内部から書き換えるため in-out 宣言が必須
        in-out property <string> message: "";

        // 縦並びレイアウト（中央揃え、上下左右にパディング、子要素間隔 16px）
        VerticalLayout {
            padding: 20px;
            spacing: 16px;
            alignment: center;

            // テキストコントロール：message プロパティの内容を表示
            Text {
                text: message;
                font-size: 20px;
            }
            // ボタン：クリックされたら message を "Hello, Slint!" に更新
            Button {
                text: "Click me";
                clicked => {
                    // 親コンポーネント（root）のプロパティを書き換える
                    root.message = "Hello, Slint!";
                }
            }
        }
    }
}

fn main() {
    // メインウィンドウのインスタンスを生成（生成に失敗した場合はパニック）
    let ui = MyApp::new().expect("Failed to create the main window");
    // ウィンドウを表示しイベントループを開始（ウィンドウが閉じられるまでブロック）
    ui.run().expect("Failed to run the main window");
}
