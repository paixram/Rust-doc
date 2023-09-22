trait Summary { // Esto es como una interfa de golang, sirve para especificar metodos que tendra un tipo
    fn summarize(&self) -> String; // cualquier tipo que tenga el trait summary incluido, tendra estas funciones deifnidas
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn notify(item: &impl Summary) { // Parametro que implementa el summary rasgo; usamos la impl Trasit sintaxis
    println!("Breaking News! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking New! {}", item.summarize());
}

/*
 fn notify(item: &(impl Summary + Display))
 */

fn notify3<T: Summary + Display>(item: &t){}; // El "+" quiere decir que aparte de los rasgos de Summary tambien 
// se implementaran los de Display, va a tener los 2
//
// limites de rasgos Con where clausulas
fn some_function<T, U>(t: &T,u: &U) -> i32
    where T: Display + Clone,
          U: clone + Debug  // los limites que aplica a los genericos
{

}

// Devolver un tipo que implemente el rasgo
fn returns_summarizable() -> impl Summary { // devuelve algun tipo que implemente la Summary rasgo sin nombrar el tipo concreto
    Tweet {
        username: String::from("Hoire_ebboks"),
        content: String::from("a porsche"), // pero no se puede devolver diferentes tipos por ahora
        reply: false,
        retweet: false,
    }
}
