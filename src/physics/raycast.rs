use super::{
    Face,
    Vec3,
    Line,
    Cube
};

fn vec3_face(a: Vec3, b: Vec3) -> Face {
    [
        b[0] - a[0] < 0.0,
        b[1] - a[1] < 0.0,
        b[2] - a[2] < 0.0
    ]
}

fn vec3_diff(a: Vec3, b: Vec3) -> Vec3 {
    [
        b[0] - a[0],
        b[1] - a[1],
        b[2] - a[2]
    ]
}

fn line_scaled_value(line: &Line, axis: usize, constant: f32) -> [Option<f32>; 3] {
    let mut vec3 = [None; 3];
    let offset;

    // find which sign/orientation to use for the constant
    if line[0][axis] <= constant && constant <= line[1][axis] {
        offset = line[0]
    } else if line[1][axis] <= constant && constant <= line[0][axis] {
        offset = line[1]
    } else {
        return vec3 // outside line bounds
    }

    // scale the constant within the line bounds
    vec3[axis] = Some(constant);
    let diff = vec3_diff(line[0], line[1]);
    let ratio = (constant - offset[axis]) / diff[axis];

    for i in 0..3 {
        if i == axis {
            continue
        }
        
        let value = (ratio * diff[i]) + offset[i];

        if line[0][i] <= value && value <= line[1][i]
        || line[1][i] <= value && value <= line[0][i]
        {
            vec3[i] = Some(value)
        }
    }

    return vec3
}

fn cube_intercept(cube: &Cube, line: &Line) -> Option<Vec3> {
    for f in vec3_face(line[0], line[1]) {
        let i = usize::from(f);
        for j in 0..3 {
            let sequence = line_scaled_value(line, j, cube[i][j]);

            // we have an intercept
            if sequence[0].is_some()
            && sequence[1].is_some()
            && sequence[2].is_some() {

                for k in 0..3 {
                    let constant = sequence[k].unwrap();
                    
                    // intercept within cube bounds
                    if cube[0][k] <= constant && constant <= cube[1][k]
                    || cube[1][k] <= constant && constant <= cube[0][k]
                    {
                        return Some([
                            sequence[0].unwrap(),
                            sequence[1].unwrap(),
                            sequence[2].unwrap()
                        ])
                    }

                }
            }
        }
    }
    None
}