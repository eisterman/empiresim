use raylib::RaylibHandle;

pub trait RLDrawable {
    fn draw(&self, rl: &mut RaylibHandle);
}
