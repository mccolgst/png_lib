use std::str;
use std::io::Read;
use std::fs::File;


fn convert_u8s_to_u64(values: &[u8]) -> u64 {
    values.iter().fold(0, |x, &i| x << 4 | i as u64)
}

fn main(){
	let len_size: usize = 4;
	let type_size: usize = 4;
	let crc_size: usize = 4;
	let mut index: usize = 8;

	let mut done = false;
	let mut data = Vec::new();
	let mut f = File::open("flag.png").unwrap();
	f.read_to_end(&mut data).expect("Unable to read data");
	println!("file is {} bytes long", data.len());

	while !done {
		// first chunk starts at position 8
		// the first 4 bytes are chunk length, n
		// then next 4 bytes are chunk type 
		// next n bytes are data
		// last next 4 bytes are CRC
		// IHDR means it's the first chunk of the PNG

		let chunk_data_size = convert_u8s_to_u64(&data[index..index + len_size]) as usize;
		let chunk_type = str::from_utf8(&data[index + len_size..index+len_size+type_size]).unwrap();
		
		println!("chunk type {} is IHDR: {}", chunk_type, chunk_type == "IHDR");
		let new_index = index + len_size + type_size + chunk_data_size + crc_size;

		println!("size of the data in this chunk {}", chunk_data_size);

		let chunk = &data[index..new_index];
		for datum in chunk {
			println!("chunk datum {}", datum);
		}
		println!("===================END OF CHUNK===================");
		index = new_index;
		done = true;
	}

}