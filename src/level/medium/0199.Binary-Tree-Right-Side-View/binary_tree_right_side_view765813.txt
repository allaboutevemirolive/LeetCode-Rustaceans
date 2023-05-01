// https://leetcode.com/problems/binary-tree-right-side-view/solutions/765813/idiomatic-rust-0-ms-solution-using-bfs/
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
	// Create the vector that we will return, notice rust compiler can figure out the type
	let mut ans = vec![]; 
	
	// Create Vec<Rc<RefCell<TreeNode>> we will use to do our BFS
	// We no longer use Option<T> because it we will only push onto this vec
	// if the enum is some (which we will enforce with pattern matching.

	let mut bfs = vec![]; 
	
	// First example of pattern matching. If root contains something,
	// we'll push the inner part of the enum onto the back of our bfs queue.
	// If it contains nothing, we'll do nothing a.k.a. ()
	match root {
		Some(x) => bfs.push(x),
		None => ()
	}
    
	// main loop of BFS, keep going if there is something in the queue.
	while bfs.len() != 0 {
		// First think we want to do is get the value of the TreeNode at the rightmost val
		// of the bfs vec
		ans.push(bfs[bfs.len()-1].borrow().val);
        // Note we have to borrow the node to get access to follow the smart pointer to 
		// the struct itself.
		
		// now we're ready to find the next row/level
		let mut row = vec![];
		for node in bfs.iter() {
			// another match statement as before, except this time we borrow the left value as a reference.
			match node.borrow().left.as_ref() {
				// only push onto row if the node actually contains something.
				Some(x) => row.push(Rc::clone(x)), 
				None => (),
			};
			// We also have to clone the pointer, as opposed to before. Why? Both the TreeNode and our row will 
			// point to the leaf in the tree. That's a problem because rust worries about things like double
			// free errors. You need to do the reference counting yourself, and the memory will only be deallocated
			// when there are no strong references to a particular struct/object. 
			// This is basically manual garbage collection and does incur a runtime penalty
			
			// Nothing fancy rustwise here. just the same thing 
			match node.borrow().right.as_ref() {
				Some(x) => row.push(Rc::clone(x)),
				None => (),
			};
		};
		// make the queue equal to the row we just built, and go back to the while loop.
		bfs = row;
	}
	// return ans
	ans
}