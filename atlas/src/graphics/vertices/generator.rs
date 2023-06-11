use super::{
    attributes::{Attribute, Normal, Position, TexCoords},
    Shapely,
};

const PI: f32 = 3.1415926535;

impl Shapely for Attribute<Position> {
    type Attribute = Attribute<Position>;

    fn quad(width: f32, height: f32) -> Vec<Self::Attribute> {
        (0..1)
            .map(|x| {
                (0..1).map(move |y| Attribute::<Position> {
                    data: [width - x as f32 * width, height - y as f32 * height, 0.0],
                })
            })
            .flatten()
            .collect()
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        (0..3)
            .map(|j| {
                (0..8).map(move |i| Attribute::<Position> {
                    data: [
                        if ((i >> 0) & 1) == 1 { side } else { -side },
                        if ((i >> 1) & 1) == 1 { side } else { -side },
                        if ((i >> 2) & 1) == 1 { side } else { -side },
                    ],
                })
            })
            .flatten()
            .collect()
    }

    fn sphere(radius: f32, detail: u32) -> Vec<Self::Attribute> {
        (0..detail)
            .map(|y| {
                let y_angle = y as f32 * PI / detail as f32;
                (0..detail).map(move |x| {
                    let x_angle = x as f32 * 2.0 * PI / detail as f32;
                    Attribute::<Position> {
                        data: [
                            radius * x_angle.cos() * y_angle.cos(),
                            radius * y_angle.sin(),
                            radius * x_angle.sin() * y_angle.cos(),
                        ],
                    }
                })
            })
            .flatten()
            .collect()
    }
}

impl Shapely for Attribute<Normal> {
    type Attribute = Attribute<Normal>;

    fn quad(width: f32, height: f32) -> Vec<Self::Attribute> {
        todo!()
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        todo!()
    }

    fn sphere(radius: f32, detail: u32) -> Vec<Self::Attribute> {
        todo!()
    }
}

impl Shapely for Attribute<TexCoords> {
    type Attribute = Attribute<TexCoords>;

    fn quad(width: f32, height: f32) -> Vec<Self::Attribute> {
        todo!()
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        todo!()
    }

    fn sphere(radius: f32, detail: u32) -> Vec<Self::Attribute> {
        todo!()
    }
}
