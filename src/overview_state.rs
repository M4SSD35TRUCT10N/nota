use orbtk::prelude::*;

use crate::{
    base_state::BaseState,
    data::{NoteList, NoteOverview},
    keys::*,
};

/// Actions that can execute on the overview.
#[derive(Debug, Copy, Clone)]
pub enum Action {
    NewEntry,
    OpenNoteList(usize),
}

/// Handles the requests of the `OverviewView`.
#[derive(Default, AsAny)]
pub struct OverviewState {
    action: Option<Action>,
    note_view: Entity,
    list_view: Entity,
}

impl BaseState for OverviewState {}

impl OverviewState {
    /// Sets a new action.
    pub fn action(&mut self, action: Action) {
        self.action = action.into();
    }

    // news a new note list.
    fn new_entry(&self, registry: &mut Registry, ctx: &mut Context) {
        ctx.widget()
            .get_mut::<NoteOverview>(PROP_NOTE_OVERVIEW)
            .push(NoteList::new("New entry"));
        self.adjust_count(ctx);
        self.save(registry, ctx);

        let index = ctx.widget().get::<NoteOverview>(PROP_NOTE_OVERVIEW).len() - 1;
        ctx.get_widget(self.note_view).set("create", true);
        self.open_note_list(ctx, index);
    }

    // Adjusts the note list count.
    fn adjust_count(&self, ctx: &mut Context) {
        let count = ctx.widget().get::<NoteOverview>(PROP_NOTE_OVERVIEW).len();
        ctx.widget().set("count", count);
    }

    // opens a note list.
    fn open_note_list(&self, ctx: &mut Context, index: usize) {
        ctx.get_widget(self.note_view)
            .set("list_index", Some(index));
        self.navigate(self.note_view, ctx);
    }
}

impl State for OverviewState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.list_view = ctx
            .entity_of_child(ID_OVERVIEW_ITEMS_WIDGET)
            .expect("OverviewState.init: Items widget child could not be found.");
        self.note_view = (*ctx.widget().get::<u32>("note_view")).into();

        if let Ok(notes) = registry
            .get::<Settings>("settings")
            .load::<NoteOverview>(PROP_NOTE_OVERVIEW)
        {
            ctx.widget().set(PROP_NOTE_OVERVIEW, notes);
        }

        self.adjust_count(ctx);
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::NewEntry => {
                    self.new_entry(registry, ctx);
                }

                Action::OpenNoteList(index) => {
                    self.open_note_list(ctx, index);
                }
            }
        }

        self.action = None;
    }
}
