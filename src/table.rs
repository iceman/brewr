mod style;

use tabled::{
	builder::Builder,
	settings::Padding,
};

pub use style::Style;

/// Build a table from slice, array or vector
pub fn from_columns<O, I, S>(columns: O, style: Style) -> String
where
	O: AsRef<[I]>,
	I: AsRef<[S]>,
	S: AsRef<str>,
{
	let columns = columns.as_ref(); // convert to slice of cols
	let row_len = columns[0].as_ref().len();

	let mut builder = Builder::with_capacity(row_len, columns.len());

	for i in 0..row_len {
		builder.push_record(
			columns
				.iter()
				.map(|col| col.as_ref()[i].as_ref()) // convert col to slice and index of col to &str
				.collect::<Vec<&str>>(),
		);
	}

	builder
		.build()
		.with(Padding::new(0, 4, 0, 0))
		.with(style.theme())
		.to_string()
}
