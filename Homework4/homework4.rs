#[derive(Debug)]
pub struct Tree<T> {

    root: Option<T>,  //Option<Box<Tree<T>>>,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,

}


impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree {
            root: None,
            left: None,
            right: None,

        }
//        unimplemented!()
    }
    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {

        match self.root {
            None => {
            //    self = &mut Tree::new();
                self.root = Some(key);
    //            self.root = Some(Box::new(Tree::new()));
                return true;
            },
            Some(ref mut x) => {
        //        let mut next = self.root.unwrap();
        //        let temp = next;
                if *x == key {
                    return false;
                }
                else if key < *x { //left
                    match self.left {
                        None => {
                            let mut new_tree = Box::new(Tree::new());
                            new_tree.root = Some(key);
                            self.left = Some(new_tree);
                            return true;
                        },
                        Some(ref mut l) => {
                            return Tree::insert(l, key);
                        //    l.insert(key);
                        },
                    };

                } else { //right
                    match self.right {
                        None => {
                            let mut new_tree = Box::new(Tree::new());
                            new_tree.root = Some(key);
                            self.right = Some(new_tree);
                            return true;
                        },
                        Some(ref mut r) =>{
                            return Tree::insert(r, key);
                        //    r.insert(key);
                        },
                    };

                }
            },
        }

//        unimplemented!()
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {

        match self.root {
            None => {
                return false;
            },
            Some(ref x) => {
                if *x == *key {
                    return true;
                } else if *x > *key {
                    match self.left {
                        None => return false,
                        Some(ref l) => {
                            return Tree::find(l, key);
                        },
                    }
                } else {
                    match self.right {
                        None => return false,
                        Some(ref r) => {
                            return Tree::find(r, key);
                        },
                    }
                }
            },

        }

    //    unimplemented!()
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {

        let mut stack: Vec<&T> = Vec::new();
        match self.root {
            None => {;}, //return stack,
            Some(ref x) => {
                stack.push(x);
            },
        };
        match self.left {
            None => {;}, //return stack,
            Some(ref l) => {
                let mut x = Tree::preorder(l);
                stack.append(&mut x);
            },
        };
        match self.right{
            None => {;}, // return stack,
            Some(ref r) => {
                let mut y = Tree::preorder(r);
                stack.append(&mut y);
            },
        };
        return stack;

    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {

        let mut stack: Vec<&T> = Vec::new();
        match self.left {
            None => {;}, //return stack,
            Some(ref l) => {
                let mut x = Tree::inorder(l);
                stack.append(&mut x);
            },
        };
        match self.root {
            None => {;}, //return stack,
            Some(ref x) => {
                stack.push(x);
            },
        };
        match self.right{
            None => {;}, // return stack,
            Some(ref r) => {
                let mut y = Tree::inorder(r);
                stack.append(&mut y);
            },
        };
        return stack;
        //unimplemented!()
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {

        let mut stack: Vec<&T> = Vec::new();

        match self.left {
            None => {;}, //return stack,
            Some(ref l) => {
                let mut x = Tree::postorder(l);
                stack.append(&mut x);
            },
        };
        match self.right{
            None => {;}, // return stack,
            Some(ref r) => {
                let mut y = Tree::postorder(r);
                stack.append(&mut y);
            },
        };
        match self.root {
            None => {;}, //return stack,
            Some(ref x) => {
                stack.push(x);
            },
        };
        return stack;
    }
}

















pub struct Tree<T> {

    root: Option<T>,  //Option<Box<Tree<T>>>,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
//    key: T
    //self.node
}


impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree {
            left: None,
            right: None,
            root: None,
        }
//        unimplemented!()
    }
    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {

        match self.root {
            None => {
            //    self = &mut Tree::new();
                self.root = Some(key);
    //            self.root = Some(Box::new(Tree::new()));
                return true;
            },
            Some(ref mut x) => {
                if *x == key {
                    return false;
                }
                else if key < *x { //left
                    match self.left {
                        None => {
                            let mut new_tree = Tree::new();
                //            self = &mut Tree::new();
                            new_tree.root = Some(key);
                            return true;
                        },

                        Some(ref mut l) => {
                            let left = Tree::insert(l, key);
                            return left;
                        },
                    };

                } else { //right
                    match self.right {
                        None => {
                            let mut new_tree = Tree::new();
                    //        self = &mut Tree::new();
                            new_tree.root = Some(key);
                            return true;
                        },
                        Some(ref mut r) =>{
                            let right = Tree::insert(r, key);
                            return right;
                        },
                    };

                }
            },
        }

//        unimplemented!()
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {

        match self.root {
            None => {
                return false;
            },
            Some(ref x) => {
                if x == key {
                    return true;
                } else if x < key {
                    match self.left {
                        None => return false,
                        Some(ref l) => {
                            let left = Tree::find(l, key);
                                return left;
                            },
                        };
                    } else {
                        match self.right {
                            None => return false,
                            Some(ref r) => {
                                let right = Tree::find(r, key);
                                return right;
                            },
                        };
                    }
                },

            }

        //    unimplemented!()
        }

        /// Returns the preorder traversal of the tree.
        pub fn preorder(&self) -> Vec<&T> {

            let mut stack: Vec<&T> = Vec::new();
            match self.root {
                None => return stack,
                Some(ref x) => {
                    stack.push(x);
                    match self.left{
                        None => return stack,
                        Some(ref l) => Tree::preorder(l),
                    };
                    match self.right{
                        None => return stack,
                        Some(ref r) => Tree::preorder(r),
                    };
                },
            }
            return stack;

        //    unimplemented!()
        }
        /// Returns the inorder traversal of the tree.
        pub fn inorder(&self) -> Vec<&T> {

            let mut stack: Vec<&T> = Vec::new();
            match self.left {
                None => return stack,
                Some(ref l) => Tree::inorder(l),
            };
            match self.root {
                None => return stack,
                Some(ref x) => {
                    stack.push(x);
                    match self.right{
                        None => return stack,
                        Some(ref r) => Tree::inorder(r),
                    };
                },
            }
            return stack;
            //unimplemented!()
        }

        /// Returns the postorder traversal of the tree.
        pub fn postorder(&self) -> Vec<&T> {

            let mut stack:Vec<&T> = Vec::new();
            match self.left {
                None => return stack,
                Some(ref l) => Tree::postorder(l),
            };
            match self.right {
                None => return stack,
                Some(ref r) => Tree::inorder(r),
            };
            match self.root {
                None => return stack,
                Some (ref x) => stack.push(x),
            }
            stack
            //unimplemented!()
        }
    }
