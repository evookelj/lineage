//THANKS TO THE HELPFUL PEOPLE AT /r/rust for helping me learn

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;

fn plot(x: usize, y: usize, screen: &mut [[[i32; 3]; 500]; 500]) {
	let y2 = 250+(y/2) as usize;
	let yf = (499-y2 as isize).abs() as usize;
	let xf = 250+(x/2) as usize;
	screen[yf][xf] = [255,255,255];
}

fn line(x0: usize, y0: usize, x1: usize, y1: usize, screen: &mut [[[i32; 3]; 500]; 500]) {
	let mut x = x0 as usize;
	let mut y = y0 as usize;
	if y0>y1 || x0>x1 {
		return line(x1,y1,x0,y0,screen);
	}
	let a = 2*(y1-y0) as isize;
	let b = -2*(x1-x0) as isize;
	let mut d: isize = 2*a+b;
	while x < x1 {
		plot(x,y, screen);

		if d>0 {
			y += 1;
			d += b;
		}
		x += 1;
		d += a;
	}
	println!("Drew line from ({},{}) to ({},{})", 
		x0, y0, x1, y1);
}

fn get_num(which: &'static str) -> usize {
	println!("Input your desired {} [0,499]", which);
	let mut num = String::new();
	io::stdin().read_line(&mut num).expect("Failed to read line");

	let num: usize = num.trim().parse()
        .expect("Please type a number on [0,499]!");
    println!("{}: {}", which, num);

    return num;
}

fn draw_line(x0: usize, y0: usize, x1: usize, y1: usize, screen: &mut [[[i32; 3]; 500]; 500]) {
	let dx: isize = (x1 as isize)-(x0 as isize) as isize;
	let dy: isize = (y1 as isize)-(y0 as isize) as isize;

	if (dx < 0 && dy > 0) || //slope neg
		(dx > 0 && dy < 0) ||
		(dy > dx) {
		println!("Slope must be >= 0 and < 1. Line cannot be drawn at this time.");
	} else {
		line(x0,y0,x1,y1, screen);
	}
}

fn user_coords(screen: &mut [[[i32; 3]; 500]; 500]) -> bool {
	let x0 = get_num("x0");
	let y0 = get_num("y0");
	let x1 = get_num("x1");
	let y1 = get_num("y1");

	draw_line(x0,y0,x1,y1, screen);

	println!("Type something and enter to add a line. Otherwise, just enter");
	let mut resp = String::new();
	io::stdin().read_line(&mut resp).expect("Failed to read line");
	return resp.len()>1;
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

	//line(0,250,250,400,&mut screen);
	//line(8,22,499,499,&mut screen);
	loop {
		if !user_coords(&mut screen) { break; }
	}

	for i in 0..500 {
		for j in 0..500 {
			match file.write_all(format!("{} {} {}\n",screen[i][j][0],screen[i][j][1],screen[i][j][2]).as_bytes()) {
				Err(why) => panic!("Error writing pixel {} {} because {}", i, j, why.description()),
				Ok(_) => (),
			};
		}
	}
	println!("Finished writing to img.ppm");
}