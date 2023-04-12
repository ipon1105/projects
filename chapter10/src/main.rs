fn longest<'str>(a: &'str str, b: &'str str) -> &'str str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn a_a_r_p(&self, a: &str) -> &str {
        self.part
    }
}

fn main () {
    // // Время жизни (lifetime)
    // let r;          //---+
    // {                     //   |
    //     let x = 5;   //-+ |
    //     r = &x;           // | |
    // }                     //-+ |
    // println!("r = {r}");  //   |
    //                       //---+

    // Анализатор заимствований
    // Аннотация времени жизни
    // &i32,
    // &'apple i32,
    // &'apple mut i32, 

    // Статическое время жизни
    // let string: &'static str = "1234567";

    // Обобщённые времена жизни
    // let str1 = "string one is cool";
    // let result;
    // {
    //     let str2 = "string two is cooler".to_string();
    //     result = longest(&str1, str2.as_str());
    // }
    // println!("The long string is: '{result}'");

    // Время жизни в структурах
    let text = String::from("Call me. Please. I love you.");
    let first_sentence = text.split('.').next().expect("Error here");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    // Правила времени жизни
    // fn foo<'a, 'b>(a: &'a str, b: &'b str)
    // fn foo<'a>(a: &'a str) -> &'a str
    // fn foo<'a>(&'a self, a: &'a str)


}