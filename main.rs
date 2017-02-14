//THANKS TO THE HELPFUL PEOPLE AT /r/rust for helping me learn

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;

fn plot(x: i32, y: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let y2 = (250+y) as usize;
	let yf = (499-y2 as i32).abs() as usize;
	let xf = (250+x) as usize;
	screen[yf][xf] = color;
}

fn line1(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 {
		return line1(x1,y1,x0,y0,screen,color);
	}
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a+b;
	while x < x1 {
		plot(x,y, screen, color);

		if d>0 {
			y += 1;
			d += b;
		}
		x += 1;
		d += a;
	}
}

fn line2(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 {
		return line2(x1,y1,x0,y0,screen,color);
	}
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*b+a;
	while y < y1 {
		plot(x,y, screen,color);

		if d<0 {
			x += 1;
			d += a;
		}
		y += 1;
		d += b;
	}
}

fn line7(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 {
		return line2(x1,y1,x0,y0,screen,color);
	}
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = a-(2*b);
	while y > y1 {
		plot(x,y, screen,color);

		if d>0 { //bc deltay = A = negative
			x += 1;
			d += a;
		}
		y -= 1;
		d -= b;
	}
}


fn line8(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0 as i32;
	let mut y = y0 as i32;
	if x0>x1 {
		return line8(x1,y1,x0,y0,screen,color);
	}
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a-b;
	while x < x1 {
		plot(x,y,screen,color);

		if d<0 {
			y -= 1;
			d -= b;
		}
		x += 1;
		d += a;
	}
}

fn get_num(which: &'static str, min: i32, max: i32) -> i32 {
	println!("Input your desired {} [{},{}]", which,min,max);
	let mut num = String::new();
	io::stdin().read_line(&mut num)
		.expect("Failed to read line");

	let num: i32 = num.trim().parse()
        .expect("Failed to read number");

    if (num>max) || (num<min) {
    	println!("{} not in range [{},{}]. Try again",num,min,max);
    	return get_num(which,min,max);
    }
    println!("{}: {}", which, num);

    return num;
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let dx: f64 = (x1 as f64)-(x0 as f64) as f64;
	let dy: f64 = (y1 as f64)-(y0 as f64) as f64;
	if dx<0.0 {
		draw_line(x1,y1,x0,y0,screen,color);
	}

	let m = dy/dx;

	if (dy==0.0) && (dx==0.0) {
		return ;
	}
	if (m >= 0.0) && (m < 1.0) {
		line1(x0,y0,x1,y1,screen,color);
	} else if m>=1.0 {
		line2(x0,y0,x1,y1,screen,color);
	}  else if (m <= 0.0) && (m > -1.0) {
		line8(x0,y0,x1,y1,screen,color);
	} else if m<=-1.0 {
		line7(x0,y0,x1,y1,screen,color);
	} else {
		println!("Should never reach this");
	}
}

fn user_coords(screen: &mut [[[u32; 3]; 500]; 500]) -> bool {
	let r = get_num("red",0,255) as u32;
	let g = get_num("green",0,255) as u32;
	let b = get_num("blue",0,255) as u32;
	let color: [u32; 3] = [r,g,b];

	let x0 = get_num("x0",-250,250);
	let y0 = get_num("y0",-250,250);
	let x1 = get_num("x1",-250,250);
	let y1 = get_num("y1",-250,250);

	draw_line(x0,y0,x1,y1, screen, color);

	println!("Type something and enter to add a line. Otherwise, just enter");
	let mut resp = String::new();
	io::stdin().read_line(&mut resp).expect("Failed to read line");
	return resp.len()>1;
}

fn img(screen: &mut [[[u32; 3]; 500]; 500]) {
	let mut i:i32 = -250;
	let mut j:i32 = -250;
	while i<251 {
		while j<251 {
			let r = (i.abs()%255) as u32;
			let g = (j.abs()%255) as u32;
			let b = ((i*j)%255) as u32;
			let color: [u32; 3] = [r,g,b];
			draw_line(i,(i-j)%250,j,(j-i)%250,screen,color);

			let r = ((i/(j+1))%255) as u32;
			let g = ((j/(i+1))%255) as u32;
			let b = ((r*g)%255) as u32;
			let color: [u32; 3] = [r,g,b];
			draw_line(((i-j)%250),i*-1,((j-i)%250),j*-1,screen,color);
			j += 10;
		}
		i += 10;
		j=-250;
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
	let mut screen: [[[u32; 3]; 500]; 500] = [[[0; 3]; 500]; 500];

 // //USER INPUT
	// loop {
	// 	if !user_coords(&mut screen) { break; }
	// } 
	img(&mut screen);

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