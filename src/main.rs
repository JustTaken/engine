use engine::renderer::wayland;

pub fn main() {
    wayland::display("Engine name", 1920, 1080).unwrap();
}
