//!
//!Simple library that generates point coordinates for drawing each digit of a decimal number.
//!
//!For example the digit "2" would return the x/y coordinates for each 2 in the below picture:
//!
//!```
//!
//!..2222
//! 22  22
//!    22
//!   22
//! 222222
//!```
//!

static MISCSTR:&'static str = "
    0
   000
  00000
 0000000
   000

   000
 0000000
  00000
   000
    0  

 00000   0000
00  00  00  00
0       00  00
0  000  00  00
 00000   0000

  0000    0000   00     0000   00000
 00  00  00  00  00    00  00  00  00
 00      00  00  00    00  00  00  00
 00  00  00  00  00    00  00  00000
  0000    0000   00000  0000   00  00

 0000   0000  0000   00  00   0000       000
 00  0  00    00 00  00  00  00  0   0  0   0
 00  0  0000  0000   00  00  0          0   0
 00  0  00    00 00  00  00  0  00   0  0   0
 0000   0000  0000    0000    0000       000

 0000   0000  0000   00  00   0000      0000
 00  0  00    00 00  00  00  00  0   0    00
 00  0  0000  0000   00  00  0            00
 00  0  00    00 00  00  00  0  00   0    00
 0000   0000  0000    0000    0000      000000

";
	
static NUMSTR:&'static str = "
 0000
00  00
00  00
00  00
 0000

1111
  11
  11
  11
111111

 2222
22  22
   22
  22
222222

 3333
33  33
   333
33  33
 3333

44  44
44  44
444444
    44
    44

555555
55
55555
    55
55555

 6666
66
66666
66  66
 6666

777777
   77
  77
 77
77

 8888
88  88
 8888
88  88
 8888

 9999
99  99
 99999
    99
 9999
 ";


mod digit{

	pub struct DigitIter{
		acc:usize,
		rest:usize,
		num_digit_left:usize
	}
	impl Iterator for DigitIter{
		type Item=usize;
		fn next(&mut self)->Option<Self::Item>{
			if self.num_digit_left==0{
				return None
			}
			//let v=10usize.pow(i as u32);
			let digit=self.rest/self.acc;
			self.rest=self.rest%self.acc;
			self.acc/=10;

			self.num_digit_left-=1;
			Some(digit)
		}
	}

	pub fn get_dec_digit(num:usize)->DigitIter{

		//10^x=num
		let start_10=(num as f64).log10().floor() as u32 +1;
		
		let mut acc=10usize.pow(start_10);
		acc/=10;
		DigitIter{acc,rest:num,num_digit_left:start_10 as usize}
	}

}

fn get_str_digit(num:usize)->Vec<(usize,usize)>{
	assert!(num<10);
	let b=NUMSTR.lines().skip(1).skip(6*num).take(6);

	let mut vec=Vec::new();
	for (y,line) in b.enumerate(){
		for (x,c) in line.chars().enumerate(){
			if c==num.to_string().chars().next().unwrap(){
				vec.push((x,y));
			}
		}
		//println!("{:?}",line);
	}
	vec
}


///Create some misc symbols. Look at static strings in the source code 
///to figure out which ones.
pub fn get_misc(num:usize)->Vec<(usize,usize)>{
	assert!(num<6);
	let b=MISCSTR.lines().skip(1).skip(6*num).take(6);

	let mut vec=Vec::new();
	
	for (y,line) in b.enumerate(){

		for (x,c) in line.chars().enumerate(){
			if c=="0".chars().next().unwrap(){
				vec.push((x,y));
			}
		}
	}
	vec
}


///Pass in a number.
///Returns digits in decimal in coordinate form.
///x grows rightwards.
///y grows downwards.
///one unit is one ascii character.
pub fn get_coords(num:usize)->Vec<Vec<(usize,usize)>>{
	let mut vec=Vec::new();
	
	for digit in digit::get_dec_digit(num){
		vec.push(get_str_digit(digit));
	}
	vec
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
