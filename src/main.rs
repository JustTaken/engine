use engine::renderer::wayland;

pub fn main() {
    wayland::display().unwrap();
}
