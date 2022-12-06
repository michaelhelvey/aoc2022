// mod day_1;
// mod day_2;
mod day_3;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    // day_1::run_part_2()?;
    day_3::run()
}
