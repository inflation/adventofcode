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

        let input = include_str!(concat!("../input/", $year, "/day", $day, ".txt"));

        paste::paste! {
            println!(
                "Answer: {:#?}",
                crate::[<y $year>]::[<day $day>]::solve(&input)?
            );
        }
    };
}
