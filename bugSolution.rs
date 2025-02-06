fn main() {
    let mut x = 5;

    // Solution 1: Use a mutable reference only when needed
    {  
        let y = &mut x; // Single mutable borrow within this scope.
        *y = 10; 
    }
    
    { 
        let z = &mut x; // Another single mutable borrow
        *z = 20;
    }

    println!("x = {}", x); // Now this outputs 20 because the borrows are handled correctly 

    // Solution 2: Using a RefCell for interior mutability (for more complex cases)
    use std::cell::RefCell;

    let x = RefCell::new(5);
    let y = x.borrow_mut(); // Borrow the value mutably
    *y = 10;
    let z = x.borrow_mut(); //Another mutable borrow
    *z = 20;
    println!("x (RefCell) = {}", x.borrow());
} 