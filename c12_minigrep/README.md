https://doc.rust-lang.org/book/ch12-00-an-io-project.html

## case-sensitive search of anything that contains "to" in "poem.txt"
cargo run -- to poem.txt

## case-insensitive search of anything that contains "to" in "poem.txt"
IGNORE_CASE=1 cargo run -- to poem.txt
