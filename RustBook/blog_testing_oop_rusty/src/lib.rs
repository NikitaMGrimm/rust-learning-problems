pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            pre_approved: false,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    pre_approved: bool,
}

impl PendingReviewPost {
    pub fn approve(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            pre_approved: true,
        }
    }

    pub fn final_approve(self) -> Result<Post, &'static str> {
        match self.pre_approved {
            true => Ok(Post {
                content: self.content,
            }),
            false => Err("Can't give final approval to post that isn't pre-approved!"),
        }
    }
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
