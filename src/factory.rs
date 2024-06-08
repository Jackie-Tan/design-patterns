pub trait Button {
    fn render(&self);
    fn on_click(&self);
}
struct WindowsButton {}
struct HTMLButton {}
impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering Windows button");
    }

    fn on_click(&self) {
        println!("Clicking Windows button")
    }
}
impl Button for HTMLButton {
    fn render(&self) {
        println!("Rendering HTML button");
    }

    fn on_click(&self) {
        println!("Clicking HTML button")
    }
}

pub trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;
    fn render(&self) {
        let ok_button: Box<dyn Button> = self.create_button();
        ok_button.on_click();
        ok_button.render();
    }
}
pub struct WindowsDialog {}
impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton {})
    }
}
pub struct WebDialog {}
impl Dialog for WebDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HTMLButton {})
    }
}
