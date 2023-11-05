fn main() {
    println!("Hello, world!");
    let mut l1: ListNode = ListNode::new(1);
    let mut l2: ListNode = ListNode::new(1);
    l1.next = Some(Box::new(ListNode::new(2)));
    l2.next = Some(Box::new(ListNode::new(3)));
    let l3 = merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
    println!("{:?}",l3);
}

#[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }
 
 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if list1 == None  && list2 ==None {
        return list1
    }else if list1 == None{
        return list2
    }else if list2 == None {
        return list1
    }
    let mut l: Option<Box<ListNode>>;
    let mut h1 = &list1;
    let mut h2 = &list2;

    if list1.as_ref().unwrap().val > list2.as_ref().unwrap().val {
        h2 = &list2.as_ref().unwrap().next;
        l = Some(Box::new(ListNode::new(list2.as_ref().unwrap().val.clone())));
    }else{
        h1 = &list1.as_ref().unwrap().next;
        l = Some(Box::new(ListNode::new(list1.as_ref().unwrap().val.clone())));
    }
    let mut lh = &mut l;

   loop {
        if h1.as_deref() == None && h2.as_deref() == None {
            break;
        }else if h1.as_deref() == None {
            
            lh.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new( h2.as_deref().unwrap().val.clone())));
            h2 = &h2.as_deref().unwrap().next;
            lh = &mut lh.as_deref_mut().unwrap().next;
            continue;
        } else if h2.as_deref() == None {
            lh.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new( h1.as_deref().unwrap().val.clone())));
            h1 = &h1.as_deref().unwrap().next;
            lh = &mut lh.as_deref_mut().unwrap().next;
            continue;
        }
        
        if h1.as_deref().unwrap().val > h2.as_deref().unwrap().val {
            lh.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new( h2.as_deref().unwrap().val.clone())));
            h2 = &h2.as_deref().unwrap().next;
            lh = &mut lh.as_deref_mut().unwrap().next;
        }else{
            lh.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new( h1.as_deref().unwrap().val.clone())));
            h1 = &h1.as_deref().unwrap().next;
            lh = &mut lh.as_deref_mut().unwrap().next;
        }
        


    }
    
    l

    }
