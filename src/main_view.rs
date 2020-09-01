use orbtk::prelude::*;

use crate::{data::NoteOverview, overview_view::OverviewView, note_view::NoteView};

widget!(MainView {
    note_overview: NoteOverview,
    count: usize,
    overview_view: u32,
    note_view: u32
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let overview_view = OverviewView::new()
            .note_view(id)
            .count(id)
            .note_overview(id)
            .build(ctx);
        let note_view = NoteView::new()
            .overview(overview_view.0)
            .note_overview(id)
            .visibility("collapsed")
            .build(ctx);

        self.name("MainView")
            .note_view(note_view.0)
            .note_overview(NoteOverview::default())
            .count(0)
            .child(overview_view)
            .child(note_view)
    }
}