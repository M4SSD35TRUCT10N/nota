use orbtk::prelude::*;

use crate::{
    data::NoteOverview,
    keys::*,
    note_state::{Action, NoteState},
};

type ListIndex = Option<usize>;

widget!(NoteView<NoteState> {
    overview: u32,
    list_index: ListIndex,
    note_overview: NoteOverview,
    count: usize,
    title: String16,
    create: bool
});

impl Template for NoteView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // list of note lists
        let items_widget = ItemsWidget::new()
            .id(ID_NOTE_ITEMS_WIDGET)
            .v_align("start")
            .items_builder(move |ctx, index| {
                let mut text = "".to_string();
                let mut selected = false;

                if let Some(list_index) = ctx.get_widget(id).clone::<ListIndex>("list_index") {
                    if let Some(note_overview) = ctx
                        .get_widget(id)
                        .get::<NoteOverview>(PROP_NOTE_OVERVIEW)
                        .get(list_index)
                    {
                        if let Some(note) = note_overview.get(index) {
                            text = note.text.clone();
                            selected = note.selected;
                        }
                    }
                }

                Grid::new()
                    .height(48)
                    .margin((8, 0))
                    .columns(
                        Columns::create()
                            .push(24)
                            .push(4)
                            .push("*")
                            .push(4)
                            .push(32),
                    )
                    .child(
                        CheckBox::new()
                            .v_align("center")
                            .selected(selected)
                            .on_changed("selected", move |ctx, entity| {
                                ctx.get_mut::<NoteState>(id)
                                    .action(Action::SelectionChanged(entity, index));
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBox::new()
                            .style(STYLE_TEXT_BOX_INLINE)
                            .text(text)
                            .v_align("center")
                            .water_mark("Insert text...")
                            .attach(Grid::column(2))
                            .on_activate(move |ctx, entity| {
                                ctx.get_mut::<NoteState>(id)
                                    .action(Action::UpdateEntry(entity, index));
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style(STYLE_BUTTON_ICON_ONLY)
                            .attach(Grid::column(3))
                            .v_align("center")
                            .icon(material_icons_font::MD_DELETE)
                            .on_mouse_down(|_, _| true)
                            .on_click(move |ctx, _| {
                                ctx.get_mut::<NoteState>(id)
                                    .action(Action::RemoveEntry(index));
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx)
            })
            .count(id)
            .build(ctx);

        let scroll_viewer = ScrollViewer::new()
            .mode(("disabled", "auto"))
            .child(items_widget)
            .build(ctx);

        let note_text_box = TextBox::new()
            .id(ID_NOTE_TEXT_BOX)
            .attach(Grid::row(4))
            .v_align("center")
            .margin((8, 0, 0, 0))
            .lost_focus_on_activation(false)
            .on_activate(move |ctx, entity| {
                ctx.get_mut::<NoteState>(id)
                    .action(Action::NewEntry(entity));
            })
            .on_changed("text", move |ctx, entity| {
                ctx.get_mut::<NoteState>(id)
                    .action(Action::InputTextChanged(entity));
            })
            .build(ctx);

        self.name("NoteView").child(
            Grid::new()
                .rows(
                    Rows::create()
                        .push(52)
                        .push(1)
                        .push("*")
                        .push(1)
                        .push("auto"),
                )
                .columns(Columns::create().push("*").push(4).push(36))
                // Content
                .child(
                    Container::new()
                        .attach(Grid::row(2))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(3))
                        .child(scroll_viewer)
                        .child(
                            ScrollIndicator::new()
                                .padding((0, 4, 4, 0))
                                .content_bounds(("bounds", items_widget))
                                .view_port_bounds(("bounds", scroll_viewer))
                                .scroll_padding(("padding", scroll_viewer))
                                .mode(scroll_viewer)
                                .build(ctx),
                        )
                        .build(ctx),
                )
                // Top Bar
                .child(
                    Container::new()
                        .style(STYLE_TOP_BAR)
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(3))
                        .child(
                            Grid::new()
                                .margin((4, 0))
                                .columns(
                                    Columns::create()
                                        .push(32)
                                        .push(4)
                                        .push("*")
                                        .push(4)
                                        .push(32),
                                )
                                .child(
                                    Button::new()
                                        .style(STYLE_BUTTON_ICON_ONLY)
                                        .icon(material_icons_font::MD_ARROW_BACK)
                                        .v_align("center")
                                        .on_click(move |ctx, _| {
                                            ctx.get_mut::<NoteState>(id)
                                                .action(Action::NavigateBack);
                                            true
                                        })
                                        .build(ctx),
                                )
                                .child(
                                    TextBox::new()
                                        .id(ID_NOTE_HEADER_TEXT_BOX)
                                        .style(STYLE_TEXT_BOX_HEADER)
                                        .attach(Grid::column(2))
                                        .v_align("center")
                                        .text(("title", id))
                                        .on_activate(move |ctx, _| {
                                            ctx.get_mut::<NoteState>(id).action(Action::Rename);
                                        })
                                        .build(ctx),
                                )
                                .child(
                                    Button::new()
                                        .attach(Grid::column(4))
                                        .style(STYLE_BUTTON_ICON_ONLY)
                                        .icon(material_icons_font::MD_DELETE)
                                        .v_align("center")
                                        .on_click(move |ctx, _| {
                                            ctx.get_mut::<NoteState>(id).action(Action::RemoteList);
                                            true
                                        })
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .style("separator")
                        .attach(Grid::row(1))
                        .attach(Grid::column_span(3))
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .style("separator")
                        .attach(Grid::row(3))
                        .attach(Grid::column_span(3))
                        .build(ctx),
                )
                // Bottom bar
                .child(
                    Container::new()
                        .style(STYLE_BOTTOM_BAR)
                        .attach(Grid::row(4))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(3))
                        .build(ctx),
                )
                .child(note_text_box)
                .child(
                    Button::new()
                        .id(ID_NOTE_ADD_BUTTON)
                        .style(STYLE_BUTTON_ICON_ONLY)
                        .attach(Grid::row(4))
                        .attach(Grid::column(2))
                        .margin((0, 0, 4, 0))
                        .enabled(false)
                        .min_size(32, 32)
                        .v_align("center")
                        .icon(material_icons_font::MD_ADD)
                        .on_click(move |ctx, _| {
                            ctx.get_mut::<NoteState>(id)
                                .action(Action::NewEntry(note_text_box));
                            true
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
