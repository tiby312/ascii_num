use axgeom::*;


	



pub struct Symbol<'a>{
	inner:&'a [Vec2<usize>]
}

impl<'a> Symbol<'a>{
	pub fn get(&self)->&[Vec2<usize>]{
		self.inner
	}

	pub fn into_inner(self)->&'a [Vec2<usize>]{
		self.inner
	}
}




pub struct SymbolTable{
	inner:Vec<Vec<Vec2<usize>>>
}


impl SymbolTable{
	pub fn lookup(&self,num:usize)->Symbol{
		Symbol{inner:&self.inner[num]}
	}
	pub fn new(table:&str,word_height:usize,num_words:usize)->SymbolTable{

		let mut words=Vec::with_capacity(num_words);

		for index in 0..num_words{
			words.push(Self::get(table,word_height,index));
		}

		SymbolTable{inner:words}
	}
	///Create some misc symbols. Look at static strings in the source code 
	///to figure out which ones.
	fn get(table:&str,word_height:usize,num:usize)->Vec<Vec2<usize>>{
		let b=table.lines().skip(1).skip(word_height*num).take(word_height);

		let mut vec=Vec::new();
		
		for (y,line) in b.enumerate(){

			for (x,c) in line.chars().enumerate(){
				if c=="0".chars().next().unwrap(){
					vec.push(vec2(x,y));
				}
			}
		}
		vec
	}
}