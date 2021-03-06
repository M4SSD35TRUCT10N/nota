use orbtk::prelude::*;

use crate::{data::NoteOverview, keys::*, overview_state::*};

widget!(
    /// Represents the start page with the overview of note lists.
    OverviewView<OverviewState> {
        note_overview: NoteOverview,
        count: usize,
        note_view: u32,
        list_dirty: bool
    }
);

impl Template for OverviewView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // list of note lists
        let items_widget = ItemsWidget::new()
            .count(id)
            .id(ID_OVERVIEW_ITEMS_WIDGET)
            .v_align("start")
            .request_update(("list_dirty", id))
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();

                if let Some(note_overview) = ctx
                    .get_widget(id)
                    .get::<NoteOverview>(PROP_NOTE_OVERVIEW)
                    .get(index)
                {
                    text = note_overview.title.clone();
                }
                Grid::new()
                    .height(48)
                    .child(
                        Button::new()
                            .height(48)
                            .style(STYLE_BUTTON_TRANSPARENT)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id)
                                    .action(Action::OpenNoteList(index));
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::new()
                            .style(STYLE_TITLE)
                            .margin((36, 0, 8, 0))
                            .text(text)
                            .v_align("center")
                            .build(ctx),
                    )
                    .build(ctx)
            })
            .build(ctx);

        let scroll_viewer = ScrollViewer::new()
            .attach(Grid::row(2))
            .mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        self.name("Overview")
            .note_overview(NoteOverview::default())
            .count(0)
            .child(
                Grid::new()
                    .rows(Rows::create().push(52).push(1).push("*").push(25))
                    // Top Bar
                    .child(
                        Container::new()
                            .style(STYLE_TOP_BAR)
                            .attach(Grid::row(0))
                            .child(
                                Grid::new()
                                    .child(
                                        TextBlock::new()
                                            .margin((36, 0, 0, 0))
                                            .style(STYLE_HEADER)
                                            .v_align("center")
                                            .h_align("start")
                                            .text("Overview")
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Container::new()
                            .style(STYLE_SEPARATOR)
                            .attach(Grid::row(1))
                            .build(ctx),
                    )
                    // Content
                    .child(scroll_viewer)
                    .child(
                        ScrollIndicator::new()
                            .attach(Grid::row(2))
                            .padding((0, 4, 4, 0))
                            .content_bounds(("bounds", items_widget))
                            .view_port_bounds(("bounds", scroll_viewer))
                            .scroll_padding(("padding", scroll_viewer))
                            .mode(scroll_viewer)
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style(STYLE_BUTTON_FLOAT)
                            .attach(Grid::row(2))
                            .attach(Grid::row_span(2))
                            .margin(8)
                            .min_size(32, 32)
                            .v_align("end")
                            .h_align("end")
                            .icon(material_icons_font::MD_ADD)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<OverviewState>(id).action(Action::NewEntry);
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
