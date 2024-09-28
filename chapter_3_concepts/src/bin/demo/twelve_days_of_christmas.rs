/**
 * 
On the first day of Christmas,
my true love gave to me
A partridge in a pear tree.
On the second day of Christmas,
my true love gave to me
Two turtle doves,
And a partridge in a pear tree.
On the third day of Christmas,
my true love gave to me
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the fourth day of Christmas,
my true love gave to me
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the fifth day of Christmas,
my true love gave to me
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the sixth day of Christmas,
my true love gave to me
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the seventh day of Christmas,
my true love gave to me
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the eighth day of Christmas,
my true love gave to me
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the ninth day of Christmas,
my true love gave to me
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the tenth day of Christmas,
my true love gave to me
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the eleventh day of Christmas,
my true love gave to me
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the twelfth day of Christmas,
my true love gave to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree!

这首歌的歌词是一个循环, 从第一天到第十二天, 每天都会重复前面的歌词, 并且在每一天都会增加一些新的歌词, 但是这些新的歌词都是在前面的歌词的基础上增加的, 所以可以使用循环来实现这首歌的歌词
到第十二天时, 歌词如下:
On the twelfth day of Christmas,
my true love gave to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree!

而第一天的歌词是:
On the first day of Christmas,
my true love gave to me
A partridge in a pear tree.

所以如果需要使用代码打印这首歌的歌词
1. 定义两个数组, 一个数组存储数字, 一个数组存储每天增加的歌词,  数组的索引对应歌词的天数, 数组的元素对应歌词的内容
注意第一天中的 `A partridge in a pear tree.` 中的 `A` 是大写的, 其他的歌词中的 `a` 是小写的, 而且其他的歌词中会存在And, 所以数组中第一个元素只包含 `And a partridge in a pear tree.` 这一句, 针对第一个元素, 提取一个单独的字符串变量, 用于打印第一天的歌词

 */
fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let extra_lyrics = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let first_day_lyrics = "A partridge in a pear tree.";

    for i in 0..days.len() {
        println!("On the {day} day of Christmas,", day=days[i]);
        println!("my true love gave to me");

        if i == 0 {
            println!("{first_day_lyrics}");
        } else {
            for j in (0..i).rev() {
                println!("{lyric}", lyric=extra_lyrics[j]);
            }
        }
    }

}
