use super::{indices::TriangleGeometry, layouts::P2TVertex};

fn pos_quad(width: f32, height: f32) -> impl Iterator<Item = [f32; 2]> {
    let (x_end, y_end) = (width * 0.5, height * 0.5);
    (0..2)
        .map(move |x| (0..2).map(move |y| {
            [x_end - width * x as f32, y_end - height * y as f32]
        }))
        .flatten()
}

fn tex_quad() -> impl Iterator<Item = [f32; 2]> {
    (0..2)
        .map(|x| (0..2).map(move |y| {
            [x as f32, y as f32]
        }))
        .flatten()
}

fn index_quad() -> Vec<TriangleGeometry> {
    vec![TriangleGeometry([0, 1, 2]), TriangleGeometry([1, 2, 3])]
}

pub fn quad(width: f32, height: f32) -> (Vec<P2TVertex>, Vec<TriangleGeometry>) {
    let pos = pos_quad(width, height);
    let tex = tex_quad();
    let vertices = pos.zip(tex).map(|(pos, tex)| P2TVertex(pos, tex)).collect();
    let indices = index_quad();
    (vertices, indices)
}
