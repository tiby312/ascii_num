
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






pub mod symbol;
pub mod digit;


pub static MISCSTR:&'static str = "
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


use crate::symbol::*;
pub struct GameSymbolTable(
	SymbolTable
);

impl GameSymbolTable{
	pub fn new()->Self{
		GameSymbolTable(SymbolTable::new(MISCSTR,6,6))
	}
  pub fn lookup(&self,num:usize)->Symbol{
    self.0.lookup(num)
  }
}