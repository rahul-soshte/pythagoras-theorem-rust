// Pythagoras Theorem
// In mathematics, the Pythagorean theorem, or Pythagoras' theorem, 
// is a fundamental relation in Euclidean geometry among the three sides of a right triangle.
// It states that the area of the square whose side
// is the hypotenuse is equal to the sum of the areas of the squares on the other two sides.

// Point of the Triangle
#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Triangle Function

#[derive(Clone, Debug, Copy)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

// Calculate square function
// Function the length of the hypotenuse from the other two sides given the
// length of latter

fn check_right_angled(the_passed_triangle: Triangle) -> bool {

    let d = (the_passed_triangle.b.x - the_passed_triangle.a.x).powi(2) + 
        (the_passed_triangle.b.y - the_passed_triangle.a.y).powi(2);

    let e = (the_passed_triangle.c.x - the_passed_triangle.b.x).powi(2) + 
    (the_passed_triangle.c.y - the_passed_triangle.b.y).powi(2);

    let f  =  (the_passed_triangle.c.x - the_passed_triangle.a.x).powi(2) + 
    (the_passed_triangle.c.y - the_passed_triangle.a.y).powi(2);

    if (d > 0.0 && e > 0.0 && f > 0.0) &&  ((d == e + f) || (e == d + f) || (f == e + d)) {

        true

    } else {
        false
    }
    

}

fn calculate_distance(a: Point, b: Point) -> f32{
    // Square root of (x2 - x1)^2 + (y2 - y1)^2

    let val = (b.x - a.x).powi(2) + (b.y - a.y).powi(2) * (1.0);
    let dist = val.sqrt();
    // println!("{}", val);
    dist

}

// Formula is a^2 + b^2 = c^2
// c = sqrt(a^2 + b^2)
fn check_hypotenuse_distance(f: f32, g:f32, h:f32) -> f32 { 
   
    let  a = f * f;
    let b = g * g;
    let c = h * h;

    // println!("{},{},{}",b, a, c);
    if a == (b + c) {
        a
    } else if b == (a + c) {
        b
    } else if c == (a + b) {
        c
    } else {
        panic!("Not a right angle");
    }
    
}

fn main() {

    //https://www.desmos.com/calculator/bi14xz1jci
    let example_triangle = Triangle {
        a: Point {x: 1.0, y: 0.0},
        b: Point  {x: 4.0, y: 0.0},
        c: Point  {x: 1.0, y: 1.0},
    };

    let result: bool = check_right_angled(example_triangle.clone());

    if result {
        let dist1 = calculate_distance(example_triangle.a, example_triangle.b);
        let dist2 = calculate_distance(example_triangle.c, example_triangle.b);
        let dist3 = calculate_distance(example_triangle.c, example_triangle.a);
        let hypotenuse_distance = check_hypotenuse_distance(dist1, dist2, dist3);
        // println!("{},{},{}", dist1, dist2, dist3);
        println!("{}", hypotenuse_distance.sqrt());
    } else {
        println!("{}","Certainly not a right angled triangle")
    }
   
}
