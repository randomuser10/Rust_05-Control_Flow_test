fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",];

    for day in 0..12 {
        println!("\n On the {:?} day of the christmas, my true love gave to me:", days[day]);

        for gift in (0..=day){
            if gift == 0 && day !=0 {
                println!("{}", gifts[0].to_lowercase());
            } else {
                println!("And {}", gifts[gift]);
            }
        }

    }
}
