
struct Article {
    author: String,
    text: String,
    //* */
}

struct Tweet {
    author: String,
    msg: String,
    //* */
}

trait Summary {
    fn sum(&self) -> String {
        format!("author is: {}", self.author())
    }

    fn author(&self) -> String;
}

impl Summary for Article { 
    fn author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn sum(&self) -> String {
        format!("{}", self.msg)
    }

    fn author(&self) -> String {
        format!("{}", self.author)
    }
}

struct A {
    x: i32,
}

use std::fmt::Display;

// fn notify(item: &(impl Summary + Display), item2: &(impl Summary + Clone)) -> String {
//     format!("author: {}; sum = {}", item.author(), item.sum())
// }

fn notify<T, U>(item: &T, item2: &U) -> String
where
    T: Summary + Display,
    U: Summary + Clone,
{
    // format!("author: {}; sum = {}", item.author(), item.sum())
    "1".to_string()
}

// fn notify<T: Summary, U: std::fmt::Display>(item: &T, item2: &U) -> String {
//     // format!("author: {}; sum = {}", item.author(), item.sum())
//     "1".to_string()
// }

fn get_summary_struct(switch: bool) -> impl Summary {
    if switch {
        Tweet {author: "Ivan".to_string(), msg: "i like you".to_string()}
    } else {
        return Article {author: "German".to_string(), text: "We win on backetball".to_string() }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if (self.x > self.y){
            print!("x > y")
        } else {
            print!("y >= x")
        }
    }
}
enum AB {
    X(),
}

// impl<T: Display> ToString for T {
    
// }

fn main() {
    // Типаж
    let tweet = get_summary_struct(true);
    let article = get_summary_struct(false);

    println!("{}", tweet.sum());
    println!("{}", article.sum());
    
    // Согласованность или Сиротское Правило (Наш Типаж + любая структура кода) || (любой Типаж + наша структура кода), но не  (Либой Типаж + Любая структура кода)
    // Поведение по умолчанию

    // println!("{}", notify(&tweet));
    // println!("{}", notify(&article));
    // Типажи как параметры, Несколько параметров и Что под этим скрывается
    // Ясные границы типажей с помощью where

    // Условная реализация методов
    let a = Pair{x: 3, y: 2};
    let b = Pair{x: AB::X(), y: AB::X()};
    a.cmp_display();

    // Общие реализации
    3.to_string();
}