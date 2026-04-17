#[derive(Debug, Clone, Copy)]
enum ViewMode {
    Icon,
    List,
    Column,
}

fn main() {
    let sidebar = ["Favorites", "Desktop", "Documents", "Downloads"];
    let mode = ViewMode::Column;
    println!("SnowDesk Finder scaffold");
    println!("sidebar: {:?}", sidebar);
    println!("mode: {:?}", mode);
    let _ = (ViewMode::Icon, ViewMode::List);
}
