use blog_testing_oop_rusty::Post;

fn main() {
    let mut post1 = Post::new();

    post1.add_text("Hello");

    let post1 = post1.request_review();
    let post1 = post1.approve();

    let mut post1 = post1.reject();
    post1.add_text(" world!");

    let post1 = post1.request_review();
    let post1 = post1.approve();

    let post1 = post1.final_approve().unwrap();

    assert_eq!(post1.content(), "Hello world!");
}
