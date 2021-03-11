// generate random number between 0 to 255
// works by making use of the fact that this range can be easily coded in binary
// running the magic_number function gives a binary number which is then converted to decimal
fn rand_255()->u8{
    let mut rand:u8 = 0;
    for i in 0..8{
        rand += magic_num::magic_number()*(2_u8.pow(i));
        
    }
    
    return rand;
}

// public function to interface with main
pub fn rand_num(lim:u8)->u8{
    // when the limit is 231 rand_255 is called repetedly until a value within range is found
    if lim == 231{
       let mut res = rand_255();
       while res>231{
           res = rand_255();
       } 
       return res;
    }

    // when the limit is 255 rand_255 is called normally 
    else if lim == 255{
        return rand_255();
    }
    
    // otherwise the program quits and gives a message
    else{
        panic!("Invalid limit");
    
    }
}

#[cfg(test)]
mod tests {
        
        use super::*;
        //test whether the rand_num function produces proper values a 1000 times
        #[test]
        fn test_range(){
            for _i in 0..1001{
                assert!(0<=rand_num(231));
                assert!(231>=rand_num(231));
                assert!(0<=rand_num(255));
                assert!(255>=rand_num(255));
            }
        }
}
