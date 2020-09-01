use orbtk::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub text: String,
    pub selected: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NoteList {
    pub title: String,
    pub list: Vec<Note>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NoteOverview {
    pub note_lists: Vec<NoteList>,
}

impl NoteList {
    pub fn new(title: impl Into<String>) -> Self {
        NoteList {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn push(&mut self, note: Note) {
        self.list.push(note);
    }

    pub fn insert_front(&mut self, note: Note) {
        self.list.insert(0, note);
    }

    pub fn remove(&mut self, index: usize) -> Note {
        self.list.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&Note> {
        self.list.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Note> {
        self.list.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl NoteOverview {
    pub fn push(&mut self, note_list: NoteList) {
        self.note_lists.push(note_list);
    }

    pub fn insert_front(&mut self, note_list: NoteList) {
        self.note_lists.insert(0, note_list);
    }

    pub fn remove(&mut self, index: usize) -> NoteList {
        self.note_lists.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&NoteList> {
        self.note_lists.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut NoteList> {
        self.note_lists.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.note_lists.len()
    }

    pub fn is_empty(&self) -> bool {
        self.note_lists.is_empty()
    }
}

into_property_source!(NoteOverview);
into_property_source!(NoteList);
