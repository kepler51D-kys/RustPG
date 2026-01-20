struct box { // distance to server for each of the 8 points of a cube
    @location(0) corners: array<f32,8>, 
}
struct out {
    @builtin(position) clip_position: vec4<f32>
}

// @vertex
// fn vs_main(in: box) -> out {

// }