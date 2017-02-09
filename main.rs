//THANKS TO THE HELPFUL PEOPLE AT /r/rust for helping me learn

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn line(x0: usize, y0: usize, x1: usize, y1: usize, screen: &mut [[[i32; 3]; 500]; 500]) {
	let x0 = 0;
	let y0 = 250;
	let x1 = 250;
	let y1 = 400;
	let mut x = x0 as usize;
	let mut y = y0 as usize;
	let a = 2*(y1-y0) as isize;
	let b = -2*(x1-x0) as isize;
	let mut d: isize = 2*a+b;
	while x < xn1 {
		screen[x][y] = [130,130,255];

		if d>0 {
			y += 1;
			d += b;
		}
		x += 1;
		d += a;
	}
}

fn main() {

	static HEADER: &'static str = "P3\n500 500 255\n";

	let path = Path::new("img.ppm");
	let display = path.display();

//create file
	let mut file = match File::create(&path) {
        Err(why) => panic!("Error creating {} because {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

//write header to file
	match file.write_all(HEADER.as_bytes()) {
		Err(why) => panic!("Error writing header because {}", why.description()),
		Ok(_) => (),
	};

	//inner array: [r,g,b] for each pixel
	let mut screen: [[[i32; 3]; 500]; 500] = [[[0; 3]; 500]; 500];

	line(0,250,250,400,&mut screen);
	line(8,22,499,499,&mut screen);

	for i in 0..500 {
		for j in 0..500 {
			match file.write_all(format!("{} {} {}\n",screen[i][j][0],screen[i][j][1],screen[i][j][2]).as_bytes()) {
				Err(why) => panic!("Error writing pixel {} {} because {}", i, j, why.description()),
				Ok(_) => (),
			};
		}
	}
}