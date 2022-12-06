// mod day_1;
mod day_2;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    // day_1::run_part_2()?;
    day_2::run()
}
