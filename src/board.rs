use bevy::prelude::Vec2;

use Field::{Free, Wall};

type Fields = Vec<Vec<Field>>;

struct Board {
    fields: Fields,
    pub width: usize,
    pub height: usize,
    field_size: Vec2
}

#[derive(Clone)]
enum Field {
    Free,
    Wall
}

impl Board {
    fn new() -> Self {
        let fields = vec![vec![Wall; 10]; 10];
        let width = fields.len();
        let height = match fields.get(0) {
            Some(vec) => vec.len(),
            None => 1
        };
        let field_size = Vec2::new(30.0, 30.0);
        Board {
            fields, width, height, field_size
        }
    }

    fn fields(&self) -> Vec<&Field> {
        let mut result = Vec::with_capacity(self.width * self.height);
        self.fields.iter()
            .for_each(|vec| vec.iter()
                .for_each(|field| result.push(field)));
        result
    }
}