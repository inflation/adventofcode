#[macro_export]
macro_rules! mods {
	($($module:literal$(,)?)*) => {
        paste::paste!{
		    $(
			    pub mod [<day $module>];
		    )*
        }
	};
}

#[macro_export]
macro_rules! run {
    ($year:literal, $day:literal) => {
        println!("Advent of Code {} Day {}", $year, $day);

        let input =
            ::std::fs::read_to_string(format_args!("input/{}/day{}.txt", $year, $day).to_string())?;

        paste::paste! {
            println!(
                "Answer: {}",
                crate::[<y $year>]::[<day $day>]::solve(&input)?
            );
        }
    };
}
