use serde::{Deserialize, Serialize};


pub struct Chapter {
    pub title: String,
    pub content: String,
}

#[cfg_attr(feature="ssr", derive(Serialize, Deserialize, Debug, Clone))]
#[cfg_attr(feature="hydrate", derive(Clone, Serialize, Deserialize, Debug))]
pub struct Novel {
    pub novel_name: String,
    pub description: String,
    pub image_url: String,
    pub owner: String,
}

impl Default for Novel {
    fn default() -> Self {
        Self {
            novel_name: String::new(),
            description: String::new(),
            image_url: String::new(),
            owner: String::new(),
        }
    }
}

#[cfg_attr(feature="ssr", derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq))]
#[cfg_attr(feature="hydrate", derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq))]
pub struct ChapterEdit {
    pub novel_name: String,
    pub chapter_title: String,
    pub bg_novel: String,
    pub current_plot: String,
    pub plot_development: String,
    pub key_role: String,
    pub writing_style: String,
    pub chapter_start: String,
    pub content: String,
}

impl Default for ChapterEdit {
    fn default() -> Self {
        Self {
            novel_name: String::new(),
            chapter_title: String::new(),
            bg_novel: String::new(),
            current_plot: String::new(),
            plot_development: String::new(),
            key_role: String::new(),
            writing_style: String::new(),
            chapter_start: String::new(),
            content: String::new()
        }
    }
}