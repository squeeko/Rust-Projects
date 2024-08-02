use ndarray::Array;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let rows = 4;
    let cols = 2;
    let mut array = Array::zeros((rows, cols));

    for mut row in array.rows_mut() {
        // creates a iterator to traverse over the rows
        for element in row.iter_mut() {
            // Return an iterator of mutable references to the elements of the array.
            *element = rng.gen_range(0.0..10.0) // generates random numbers from a range
        }
    }
    println!("Created a random array: \n {:?}", array);
}

/*
Output

Created a random array:
 [[5.066757201421268, 8.403933058788251],
 [5.125425015317376, 0.04079669966169819],
 [5.7295911981446235, 2.8188865683883813],
 [2.7884830485905487, 7.936975414490095]], shape=[4, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2
*/
