mod template;

use template::MainWindowTemplate;

use glib::{wrapper, Object};
use gtk::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window,
};
use libadwaita::Application;

wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowTemplate>)
    @extends libadwaita::ApplicationWindow, ApplicationWindow, Window, Widget,
    @implements ActionGroup, ActionMap, Accessible, Buildable,
                ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)])
    }

    // fn on_ui_state_changed(app: &Application, ParamSpec pspec) {
    //     // UI when not selecting stuff
    //     self.add_button.visible = (self.state == UiState.NORMAL || self.state == UiState.SHOWING);
    //     self.hamburger_menu_button.visible = (self.state == UiState.NORMAL || self.state == UiState.SHOWING);

    //     // UI when showing a contact
    //     self.contact_sheet_buttons.visible = (self.state == UiState.SHOWING);

    //     // Selecting UI
    //     self.select_cancel_button.visible = (self.state == UiState.SELECTING);
    //     self.selection_button.visible = !(self.state == UiState.SELECTING || self.state.editing());

    //     if self.state != UiState.SELECTING {
    //         self.left_header.title_widget = new AdWWindowTitle (_("Contacts"), "");
    //     }

    //     // Allow the back gesture when not browsing
    //     self.content_box.can_navigate_back = self.state == UiState.NORMAL || self.state == UiState.SHOWING || this.state == UiState.SELECTING;

    //     self.actions_bar.revealed = (self.state == UiState.SELECTING);
    // }

}
