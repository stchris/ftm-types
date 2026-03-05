use ftm_types::generated::*;

// cargo run --example builder

fn main() {
    // let person = Person::builder().name(vec!["James Johnson".into()]).build();
    // this fails to compile:
    //     the member `bon::__::Unset<entities::person_builder::members::id>` was not set, but this method requires it to be set
    let person = Person::builder()
        .name(vec!["James Johnson".into()])
        .id("1234".into())
        .build();
    dbg!(person);
    // prints:
    //
    // [examples/builder.rs:11:5] person = Person {
    // id: "1234",
    // schema: "Person",
    // abbreviation: None,
    // address: None,
    // ...
    // name: [
    //     "James Johnson",
    // ],
    // ...
    // }
}
