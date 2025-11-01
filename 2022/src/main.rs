pub mod day5;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;

pub use day5::*;
pub use day7::*;
pub use day8::*;
pub use day9::*;
pub use day10::*;





fn main() 
{
    let output = day10::day10_part2();
    println!("{}", output);
}
