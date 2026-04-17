#[derive(Debug)]
enum ResultKind {
    Application,
    File,
    Calculation,
}

#[derive(Debug)]
struct SearchResult {
    title: String,
    subtitle: String,
    kind: ResultKind,
}

fn query(input: &str) -> Vec<SearchResult> {
    if input.contains('+') {
        return vec![SearchResult {
            title: "2 + 2 = 4".into(),
            subtitle: "Calculator".into(),
            kind: ResultKind::Calculation,
        }];
    }

    vec![
        SearchResult {
            title: "Finder".into(),
            subtitle: "Application".into(),
            kind: ResultKind::Application,
        },
        SearchResult {
            title: format!("{input}.txt"),
            subtitle: "File in Home".into(),
            kind: ResultKind::File,
        },
    ]
}

fn main() {
    println!("SnowDesk Spotlight scaffold");
    for result in query("2+2") {
        println!("- {} [{}] ({:?})", result.title, result.subtitle, result.kind);
    }
}
