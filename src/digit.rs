use crate::symbol::*;
use axgeom::*;
use digits_iterator;
use digits_iterator::DigitsExtension;
use digits_iterator::*;


pub static NUMSTR:&'static str = "
 0000
00  00
00  00
00  00
 0000

0000
  00
  00
  00
000000

 0000
00  00
   00
  00
000000

 0000
00  00
   000
00  00
 0000

00  00
00  00
000000
    00
    00

000000
00
00000
    00
00000

 0000
00
00000
00  00
 0000

000000
   00
  00
 00
00

 0000
00  00
 0000
00  00
 0000

 0000
00  00
 00000
    00
 0000.
 ";


pub struct Digit<'a>{
	num:u8,
	inner:Symbol<'a>
}

impl<'a> Digit<'a>{
	pub fn num(&self)->usize{
		self.num as usize
	}
	pub fn get(&self)->&[Vec2<usize>]{
		self.inner.get()
	}
	pub fn into_inner(self)->&'a [Vec2<usize>]{
		self.inner.into_inner()
	}
}


pub struct DigitSymbolTable(
	SymbolTable
);


pub struct Number<'a>{
	table:&'a DigitSymbolTable,
	number:usize
}
impl<'a> Number<'a>{

	pub fn update_number(&mut self,number:usize){
		self.number=number;
	}
	pub fn get_number(&self)->usize{
		self.number
	}

	pub fn iter(&self)->DigitIter<'a>{
		let inner=self.number.digits();//digit::get_dec_digit(self.number);
		let table=self.table;
		DigitIter{inner,table}
	}
}

pub struct DigitIter<'a>{
	table:&'a DigitSymbolTable,
	inner:Digits<usize>
}
impl<'a> Iterator for DigitIter<'a>{
	type Item=Digit<'a>;
	fn next(&mut self)->Option<Self::Item>{
		match self.inner.next(){
			None=>None,
			Some(digit)=>{
				Some(self.table.lookup_digit(digit))
			}
		}
	}
}

impl<'a> DoubleEndedIterator for DigitIter<'a>{
	fn next_back(&mut self) -> Option<Self::Item>{
		match self.inner.next_back(){
			None=>None,
			Some(digit)=>{
				Some(self.table.lookup_digit(digit))
			}
		}
	}
}


impl DigitSymbolTable{
	pub fn new_default()->DigitSymbolTable{
		DigitSymbolTable(SymbolTable::new(NUMSTR,6,10))
	}

	pub fn lookup_digit(&self,num:u8)->Digit{
		assert!(num<10);
		Digit{num:num,inner:self.0.lookup(num as usize)}
	}

	pub fn lookup_number(&self,number:usize)->Number{
		Number{table:self,number}
	}


}

/*
//split a number into an iterator that returns the digits (base 10)
mod digit{
	#[derive(Clone,Copy)]
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
		fn size_hint(&self)->(usize,Option<usize>){
			(self.num_digit_left,Some(self.num_digit_left))
		}
	}


	impl DoubleEndedIterator for DigitIter{
		fn next_back(&mut self) -> Option<Self::Item>{
			let digit=self.rect%acc;

		}
	}


	impl std::iter::FusedIterator for DigitIter{}
	impl std::iter::ExactSizeIterator for DigitIter{}

	pub fn get_dec_digit(num:usize)->DigitIter{

		//10^x=num
		let start_10=(num as f64).log10().floor() as u32 +1;
		
		let mut acc=10usize.pow(start_10);
		acc/=10;
		DigitIter{acc,rest:num,num_digit_left:start_10 as usize}
	}

}
*/