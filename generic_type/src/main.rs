use generic_type::{notify, NewsArtical, SocialPost};

#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let point1 = Point { x: 20, y: 10.0 };
    let point2 = Point { x: 30, y: 40.0 };

    let point3 = point1.mixup(point2);

    println!("point 3 mixup points is : {:?}", point3);

    let news = NewsArtical {
        content: String::from("This is content of news artical"),
        location: String::from("China"),
        author: String::from("HP"),
        headline: String::from("HeadLine"),
    };

    // println!("the news summary is: {}", news.summarize());
    // notify(&news);
    let post = SocialPost {
        username: String::from("horse_ebooks"),

        content: String::from("of course, as you probably already know, people"),
        replay: false,
        repost: false,
    };

    // println!("1 new post: {}", post.summarize());
    // notify(&post);
    notify(&news, &post);

    fn add(item: &str) -> &str {
        item
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let value_add = add("123");
    println!("the value is :{}", value_add);
    let v = longest("123", "hello");
    println!("the value of v: {}", v);
}
