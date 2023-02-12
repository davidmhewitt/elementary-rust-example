use gtk::traits::BoxExt;

pub struct WelcomeView;

impl WelcomeView {
    pub fn build() -> gtk::Box {
        let toplevel = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let welcome = granite::Placeholder::builder()
            .title("Welcome")
            .description("Hello from Rust")
            .build();

        toplevel.append(&welcome);

        toplevel
    }
}
