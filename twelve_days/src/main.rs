const DAYS: [(&str, &str); 12] = [
    ("first", "A partridge in a pear tree"),
    ("second", "Two turtle doves"),
    ("third", "Three French hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five gold rings"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eigth", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("tenth", "Ten lords a-leaping"),
    ("eleventh", "Eleven pipers piping"),
    ("twelth", "Twelve drummers drumming"),
];

fn main() {
    for (index, (day, _)) in DAYS.iter().enumerate() {
        println!("On the {day} day of Christmas, my true love gave to me");
        gifts(index);
    }
}

fn gifts(day: usize) {
    match day {
        0 => {
            println!("A partridge in a pear tree");
            println!("");
        }
        1 => {
            println!("Two turtle doves");
            println!("And a partridge in a pear tree");
            println!("");
        }
        _ => {
            let (_, gift) = DAYS[day];
            println!("{gift}");
            gifts(day - 1);
        }
    }
}
