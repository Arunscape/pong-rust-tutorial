extern crate amethyst;
use amethyst::prelude::*;
struct MyState;
impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {}
}
