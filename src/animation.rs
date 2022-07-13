use bevy::prelude::*;
use std::time::Duration;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_animations)
        ;
    }
}

fn update_animations(
    time: Res<Time>,
    mut query: Query<(&mut Handle<Image>, &mut Animation)>
) {
    let delta = time.delta();
    for (mut texture, mut animation) in query.iter_mut() {
        animation.update(delta);
        *texture = animation.get_current_texture();
    }
}

/// Component that describes a running animation of some entity.
///
/// Basically, you provide a vec of image handles and an animation time. Based
/// on the elapsed time the index of the current texture is determined.
///
/// The animation can be repeatable or not. If it is not repeatable, the last texture
/// in the vector is returned forever.
#[derive(Component)]
pub struct Animation {
    num_textures: usize,
    current_texture_index: usize,
    duration_secs: f32,
    timer: Timer,
    textures: Vec<Handle<Image>>
}

impl Animation {
    pub fn new(duration_secs: f32, repeating: bool, textures: Vec<Handle<Image>>) -> Self {
        Animation {
            num_textures: textures.len(),
            current_texture_index: 0,
            duration_secs,
            timer: Timer::new(Duration::from_secs_f32(duration_secs), repeating),
            textures
        }
    }

    /// Proceed the timer and calculate the next texture index
    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        let elapsed = self.timer.elapsed_secs();

        match elapsed < self.duration_secs {
            true => {
                let relation = elapsed / self.duration_secs;
                self.current_texture_index = ((self.num_textures as f32) * relation) as usize
            },
            false => self.current_texture_index = self.num_textures - 1
        }
    }

    pub fn get_current_texture(&self) -> Handle<Image> {
        self.textures.get(self.current_texture_index).expect("the current texture index should be in range of the amount of textures").clone()
    }
}