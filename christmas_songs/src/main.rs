fn main() {
    
    let repeat_lyrics = ["And a partridge in a pear tree.",
                         "Two turtle doves,",
                         "Three French hens,",
                         "Four calling birds,",
                         "Five golden rings!",
                         "Six geese a-laying,",
                         "Seven swans a-swimming,",
                         "Eight maids a-milking,",
                         "Nine pipers piping,",
                         "Ten drummers drumming,",
                         "Eleven lords a-leaping,",
                         "Twelve ladies dancing,"];

    for i in 1..12 {
        
        get_fixed_lyrics();

        for j in (0..i).rev() {
            if i == 1 && j == 0 {
                println!("A partridge in a pear tree.");
            } else {
                println!("{}", repeat_lyrics[j]);
            }
        }
    }

}

fn get_fixed_lyrics() {
    println!("");
    println!("On the first day of Christmas,");
    println!("My true love sent to me");
}
