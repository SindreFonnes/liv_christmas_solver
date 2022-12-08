pub fn get_string_from_vedlegg () -> String {
	let text = include_str!("../data/vedlegg.txt");
	text.to_owned()
}

