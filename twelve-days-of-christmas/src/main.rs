fn main() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let lyrics: [&str; 11] = [
        "Two turtle doves, and",
        "Three French hens,",
        "Four colly birds,",
        "Five golden rings,",
        "Six geese a laying,",
        "Seven swans a swimming,",
        "Eight maids a milking,",
        "Nine drummers drumming,",
        "Ten pipers piping,",
        "Eleven ladies dancing,",
        "Twelve lords a leaping,",
    ];
    let true_love: &str = "My true love sent to me";
    let pear_tree: &str = "A partridge in a pear tree.";
    let mut lyric_count: usize = 0;

    for i in 0..=11 {
        println!("\nOn the {} day of Christmas,", days[i]);
        println!("{true_love}");

        if i == 0 {
            println!("{pear_tree}");
            continue;
        }

        for j in (0..=lyric_count).rev() {
            println!("{}", lyrics[j]);
        }
        *&mut lyric_count += 1;

        println!("{pear_tree}");
    }
}
