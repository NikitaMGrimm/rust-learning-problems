use blog_testing_oop::Post;

fn main() {
    let mut post1 = Post::new();

    post1.add_text("Hello");

    post1.request_review();
    post1.approve();

    post1.add_text(" world! (EDIT)");

    post1.reject();
    post1.add_text(" world!");

    post1.request_review();
    post1.approve();

    assert_eq!(post1.content(), "");
    post1.approve();

    assert_eq!(post1.content(), "Hello world!");
}
