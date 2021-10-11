//use portals::RemoteDesktopPage;
use gtk::prelude::*;
use widgets::portals::ScreenCastClientPage;
fn main() {
    let application = gtk::Application::new(Some("com.gitlab.screencast"), Default::default());
    application.connect_activate(|app| {
        gst::init().expect("Unable to init gstreamer");
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("First GTK Program"));
        window.set_default_size(350, 70);

        let win = ScreenCastClientPage::new();
        window.set_child(Some(&win));
        window.show();
    });
    application.run();
}
