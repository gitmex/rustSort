extern crate rand;

use rand::Rng;

fn go_verify(verify_mir: &mut [i32]){
	for i in 0..verify_mir.len()-1{
		assert!(verify_mir[i]<=verify_mir[i+1]);
	}	
}

fn go_bubble(sortier_mir: &mut [i32], factor:i32){
	let mut replaces = 0;
	let mut compares = 0;	
	let mut change = true;
	let mut counter = 0;
	
	//println!("{:?}", sortier_mir);
	while change{
		change = false;
		for i in 0..(sortier_mir.len()-1){
			compares+=1;			
			if sortier_mir[i] > sortier_mir[i+1]{		
				let mut tmp_int = sortier_mir[i];
				sortier_mir[i]=sortier_mir[i+1];
				sortier_mir[i+1]=tmp_int;
				replaces+=1;
				change=true;				
			}
		}
		if counter % factor==0 {
			print!(".");
			counter+=1;
		}
	}
	println!("Replaces {} Compares {}\n\n", replaces, compares);
}

fn go_selection(sortier_mir: &mut [i32]){
	let mut tempInt=0;
	let mut starter=sortier_mir[0];
	let mut starterpos = 0;	
	let mut switcher = false;
	let mut compares = 0;
	let mut replaces = 0;
	//println!("{:?}", sortier_mir);

	for j in 0..sortier_mir.len(){
		print!(".");
		starter=sortier_mir[j];
		starterpos=j;
		switcher = false;
		for i in j..sortier_mir.len(){
			compares+=1;			
			if (sortier_mir[i] < starter){
				starter=sortier_mir[i];
				starterpos=i;
				switcher = true;
			}		
		}	
		if switcher{
			replaces+=1;
			sortier_mir.swap(starterpos,j);
		}
	}	
	println!("Replaces {} Compares {}\n\n", replaces, compares);

}

fn go_insertion(sortier_mir: &mut [i32]){
	let mut to_sort = 0;
	let mut j = 0;		
	let mut compares = 0;
	let mut replaces = 0;


	for i in 2..sortier_mir.len(){
		to_sort=sortier_mir[i];
		j=i;

		while j > 0 && sortier_mir[j-1] > to_sort{
			compares+=1;
			replaces+=1;			
			sortier_mir[j]=sortier_mir[j-1];			
			j=j-1;
				
		}
		print!(".");
		sortier_mir[j]=to_sort;
		replaces+=1;		
	}
	println!("Replaces {} Compares {}\n\n", replaces, compares);
}

fn go_shell(sortier_mir: &mut [i32]){
	let mut compares = 0;
	let mut replaces = 0;
	let mut gap = sortier_mir.len() / 2;
	
	while gap != 0 {
		for i in gap..sortier_mir.len() {
			let mut j = i - gap;
			while sortier_mir[j] > sortier_mir[j + gap] {
				compares+=1;	
				sortier_mir.swap(j, j + gap);
				print!(".");
				replaces+=1;
				if j >= gap {
					j -= gap;
				} else {
					break;
				}
			
			}
		
		}
		gap /=2;		
	}

	println!("Replaces {} Compares {}", replaces, compares);
}

fn main() {
	let lenlen: i32=10;	
	let factor: i32= 10;    
	let mut secret_num: Vec<_> = (0..lenlen*factor).collect();
	assert_eq!(lenlen*factor,secret_num.len() as i32);

    let mut rng = rand::thread_rng();

    rng.shuffle(&mut secret_num);

	go_bubble(secret_num.as_mut_slice(), factor);
	go_verify(secret_num.as_mut_slice());
	rng.shuffle(&mut secret_num);

	go_selection(secret_num.as_mut_slice());

	rng.shuffle(&mut secret_num);

	go_insertion(secret_num.as_mut_slice());

	rng.shuffle(&mut secret_num);

	go_shell(secret_num.as_mut_slice());

}


