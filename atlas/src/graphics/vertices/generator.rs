use super::Shapely;

const PI: f32 = 3.1415926535;

struct Position;
struct TexCoords;
struct Normal;
struct Position2D;

impl Shapely for Position {
    type Attribute = [f32; 3];

    fn quad(width: f32, height: f32) -> Vec<Self::Attribute> {
        (0..1)
            .map(|x| {
                (0..1).map(move |y| [width - x as f32 * width, height - y as f32 * height, 0.0])
            })
            .flatten()
            .collect()
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        (0..3)
            .map(|j| {
                (0..8).map(move |i| {
                    [
                        if ((i >> 0) & 1) == 1 { side } else { -side },
                        if ((i >> 1) & 1) == 1 { side } else { -side },
                        if ((i >> 2) & 1) == 1 { side } else { -side },
                    ]
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
                    [
                        radius * x_angle.cos() * y_angle.cos(),
                        radius * y_angle.sin(),
                        radius * x_angle.sin() * y_angle.cos(),
                    ]
                })
            })
            .flatten()
            .collect()
    }
}

impl Shapely for Normal {
    type Attribute = [f32; 3];

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

impl Shapely for TexCoords {
    type Attribute = [f32; 2];

    fn quad(width: f32, height: f32) -> Vec<Self::Attribute> {
        (0..1)
            .map(|x| (0..1).map(move |y| [1.0 - x as f32, 1.0 - y as f32]))
            .flatten()
            .collect()
    }

    fn skybox(side: f32) -> Vec<Self::Attribute> {
        (0..3)
            .map(|j| {
                (0..8).map(|i| match j {
                    0 => [
                        if ((i >> 0) & 1) != ((i >> 2) & 1) {
                            1.0
                        } else {
                            0.0
                        },
                        if ((i >> 1) & 1) == 0 { 1.0 } else { 0.0 },
                    ],
                    1 => [
                        if ((i >> 0) & 1) == 1 { 1.0 } else { 0.0 },
                        if ((i >> 2) & 1) != ((i >> 1) & 1) {
                            1.0
                        } else {
                            0.0
                        },
                    ],
                    2 => [
                        if ((i >> 2) & 1) == ((i >> 0) & 1) {
                            1.0
                        } else {
                            0.0
                        },
                        if ((i >> 1) & 1) == 0 { 1.0 } else { 0.0 },
                    ],
                })
            })
            .flatten()
            .collect()
    }

    fn sphere(radius: f32, detail: u32) -> Vec<Self::Attribute> {
        todo!()
    }
}
