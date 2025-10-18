// 構造体が参照を保持する場合、その参照のライフタイムを明示する必要がある
struct BookChapter<'a> {
    title: &'a str,
    content: &'a str,
}

// 関数にライフタイムを付与するのみではダメで、上の構造体にもライフタイムを明記する必要がある
fn get_chapter_title<'a>(chapter: &'a BookChapter) -> &'a str {
    chapter.title
}

// 関数にライフタイムを付与するのみではダメで、上の構造体にもライフタイムを明記する必要がある
fn get_chapter_content<'a>(chapter: &'a BookChapter) -> &'a str {
    chapter.content
}

fn main() {
    let chapter_title = String::from("Rustの基礎");
    let chapter_content = String::from("この章ではRustの基本的な文法を学びます。");

    let chapter = BookChapter {
        title: &chapter_title,
        content: &chapter_content,
    };

    let title_ref = get_chapter_title(&chapter);
    println!("章のタイトル: {}", title_ref);

    let content_ref = get_chapter_content(&chapter);
    println!("章の内容: {}", content_ref);
}
