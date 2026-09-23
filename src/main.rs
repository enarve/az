mod editor;

use editor::Editor;

fn main() {
    let editor = Editor::default();
    let res = editor.run();
    match res {
        Ok(_) => (),
        Err(err) => panic!("{err:?}")
    };
    println!("Goodbye!");
}