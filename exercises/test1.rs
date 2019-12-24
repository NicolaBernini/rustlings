// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 dollars, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// I AM NOT DONE

// Put your function here!
// fn ..... {

// Don't modify this function!

fn calculate_apple_price(n: i32) -> i32 {
	let price_std:i32 = 2; 
	let price_disc:i32 = 1; 
	if n>40 {return n*price_disc;}
	return n*price_std; 
}

#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
