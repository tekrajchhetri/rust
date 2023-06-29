use examplestdlibrary::colors::{ColorString, Color};

#[test]
fn test_red_coloring() {
    let mut colorstr = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colored: "".to_string(),
    };
    colorstr.paint();
    assert_eq!(colorstr.colored, "\x1b[31mRed\x1b[0m");
}