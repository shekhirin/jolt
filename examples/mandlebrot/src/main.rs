pub fn main() {
    let (prove_is_in_mandelbrot_set, verify_is_in_mandelbrot_set) = guest::build_is_in_mandelbrot_set();

    let (output, proof) = prove_is_in_mandelbrot_set(2.0, 1.0, 50);
    let is_valid = verify_is_in_mandelbrot_set(proof);

    println!("output: {}", output);
    println!("valid: {:?}", is_valid);
}