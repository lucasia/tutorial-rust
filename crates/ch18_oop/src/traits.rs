#[cfg(test)]
mod tests {
    use crate::traits::tests::gui::Screen;
    use crate::traits::tests::gui::components::{Button, SelectBox};
    use test_log::test;

    pub mod gui {
        use log::debug;

        pub trait Draw: std::fmt::Debug {
            fn draw(&self);
        }

        #[derive(Debug)]
        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>,
        }

        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    debug!("Drawing component: {:?}", component);
                    component.draw();
                }
            }
        }

        pub mod components {
            use crate::traits::tests::gui::Draw;
            use log::debug;

            #[derive(Debug)]
            pub struct Button {
                pub(crate) _width: u32,
                pub(crate) _height: u32,
                pub(crate) _label: String,
            }

            impl Draw for Button {
                fn draw(&self) {
                    debug!("Drawing button: {:?}", self);
                }
            }

            #[derive(Debug)]
            pub struct SelectBox {
                pub _width: u32,
                pub _height: u32,
                pub _options: Vec<String>,
            }

            impl Draw for SelectBox {
                fn draw(&self) {
                    debug!("Drawing select box: {:?}", self);
                }
            }
        }
    }

    #[test]
    fn test_draw_components() {
        let screen = Screen {
            components: vec![
                Box::new(Button {
                    _width: 100,
                    _height: 100,
                    _label: "".to_string(),
                }),
                Box::new(SelectBox {
                    _width: 0,
                    _height: 0,
                    _options: vec![String::from("Yes"), String::from("No")],
                }),
            ],
        };

        screen.run();
    }
}
