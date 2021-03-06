pub mod bad_stack {

	use std::mem::replace;

	pub struct List {
    head: Link,
	}

	enum Link {
	    Empty,
	    More(Box<Node>),
	}

	struct Node {
	    elem: i32,
	    next: Link,
	}

	impl List {
		pub fn new() -> Self {
			return List { head: Link::Empty };
		}

		pub fn push(&mut self, elem: i32) {
		    let new_node = Box::new(Node {
		        elem: elem,
		        next: replace(&mut self.head, Link::Empty),
		    });

		    self.head = Link::More(new_node);
		}

		pub fn pop(&mut self) -> Option<i32> {
			match replace(&mut self.head, Link::Empty) {
				Link::Empty => None,
				Link::More(boxed_node) => {
					let node = *boxed_node;
					self.head = node.next;
					Some(node.elem)
				},
			}	
		}
	}

	impl Drop for List {
	    fn drop(&mut self) {
	        let mut cur_link = replace(&mut self.head, Link::Empty);
	        while let Link::More(mut boxed_node) = cur_link {
	            cur_link = replace(&mut boxed_node.next, Link::Empty);
	        }
	    }
	}
}