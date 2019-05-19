use std::rc::Rc;
use std::fmt::Display;
use std::fs::read;

enum Tree<T> {
    Empty,
    Node { value: T, left: Box<Tree<T>>, right: Box<Tree<T>> },
}

impl <T> Tree<T> {

    fn is_empty(&self) -> bool {
        match *self {
            Tree::Empty => true,
            _  => false,
        }
    }

//    fn is_leaf(&self) -> bool {
//        match *self {
//            Tree::Empty => false,
//            Tree::Node { ref value, ref left, ref right } =>
//                left.is_empty() && right.is_empty(),
//        }
//    }
}

/*
                8
              /   \
             5     4
            / \     \
           9   7     11
          /   / \    /
        10   1  12  3
                /
               2
*/

fn empty<T>() -> Box<Tree<T>> {
    Box::new(Tree::Empty)
}

fn leaf<T>(value: T) -> Box<Tree<T>> {
    Box::new(Tree::Node {
        value,
        left: empty(),
        right: empty(),
    })
}

fn tree<T>(value: T, left: Box<Tree<T>>, right: Box<Tree<T>>) -> Box<Tree<T>> {
    Box::new(Tree::Node {
        value,
        left,
        right,
    })
}

fn print_tree<T: Display>(tree: Box<Tree<T>>) {
    match *tree {
        Tree::Empty => {},
        Tree::Node { value, left, right, } => {
            print!("{}, ", value);
            print_tree(left);
            print_tree(right);
        },
    }
}

//TODO is there a tailrec notation?
fn clockwise<T: Display>(tree: Box<Tree<T>>, is_left_edge: bool, is_right_edge: bool) -> String {
    match *tree {
        Tree::Empty => "".to_string(),
        //TODO how to match that left and right are Tree::Empty?
//        Tree::Node { ref value, ref left, ref right } if left.is_empty() && right.is_empty() => {
        Tree::Node { ref value, ref left, ref right }
            if left.is_empty() && right.is_empty() => {
            format!("{} ", value)
        },
        Tree::Node { value, left, right } => {
            let b = if is_right_edge { format!("{} ", value) } else { "".to_string() };

            let r_is_left_edge = is_left_edge && right.is_empty(); //TODO need to do it before recursive call because of the move
            let l_is_right_edge = is_right_edge && right.is_empty();

            let r = clockwise(right, r_is_left_edge, is_right_edge);
            let l = clockwise(left, is_left_edge, l_is_right_edge);
            let a = if is_left_edge && !is_right_edge { format!("{} ", value) } else { "".to_string() };

            b + &r + &l + &a
        },
    }
}

fn main() {
    println!("Hello, world!");

    let t =
        tree(8,
             tree(5,
                  tree(9,
                       leaf(10),
                       empty()),
                  tree(7,
                       leaf(1),
                       tree(12,
                            leaf(2),
                            empty()))),
             tree(4,
                  empty(),
                  tree(11,
                       leaf(3),
                       empty())));

//    print_tree(t);

    //TODO how to avoid moving t here? Can it be converted to RC or be passed by ref?
    println!("{}", clockwise(*rt.clone(), true, true));
}
