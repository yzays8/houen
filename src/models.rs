use std::fmt;

use colored::Colorize;

pub struct MovieData {
    pub title: String,
    pub release_date: String,
    pub is_most_relevant: bool,
}

impl MovieData {
    pub fn new(title: String, release_date: String, is_most_relevant: bool) -> Self {
        Self {
            title,
            release_date,
            is_most_relevant,
        }
    }
}

impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_most_relevant {
            write!(
                f,
                "{} {} ({})",
                String::from("[Most relevant title]").green(),
                self.title,
                self.release_date.blue()
            )
        } else {
            write!(f, "{} ({})", self.title, self.release_date.blue())
        }
    }
}
