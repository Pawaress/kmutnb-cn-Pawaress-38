fn main() {
    print!(
    "Ans = {}",
    count(unique(words("this cat this bat this rat")))
)
}
fn words(x: &str) -> Vec<String> {
    x.to_string();
    let mut v: Vec<String> = x.split(' ').map(str::to_string).collect();
    v
}
fn unique(x: Vec<String>) -> Vec<String> {
    let mut bc: Vec<String> = Vec::new();
    let mut bend: Vec<String> = Vec::new();
    let mut bd: Vec<String> = Vec::new();
    let mut be: Vec<String> = Vec::new();
    let mut b: Vec<String> = x;
    let mut keeptexte = String::from("");
    let mut bool = true;
    for i in 0..b.len() {
        for j in i + 1..b.len() {
            //println!("[ {} {} ]",b[i],b[j]);
            for p in 0..bd.len() {
                //println!("[ {} ]",bd[p]);
                if b[i] == bd[p] {
                    bool = false;
                }
            }

            if (b[i] == b[j] && bool) {
                let mut text = b[i].as_str();

                let mut keeptext = String::from("");
                for k in 0..b.len() {
                    //keeptext.push_str(" ");
                    keeptext.push_str(b[k].as_str());
                }

                let mut c: Vec<String> =
                    keeptext.split(b[j].as_str()).map(str::to_string).collect();
                c.push(text.to_string());
                bc = c;
                bd.push(text.to_string());
                //b.len()=c.len();
                //v.len()=b.len();
                for p in 0..bd.len() {
                    //println!("[ {} ]",bd[p]);
                    if b[i] == bd[p] {
                        bool = false;
                    }
                }
            }
        }
        bool = true;
        //println!("")
    }
    let mut keeptext = String::from("");
    for k in 0..b.len() {
        keeptext.push_str("-");
        keeptext.push_str(b[k].as_str());
    }
    //println!("ss = {}",keeptext);
    //ss = thiscatthisbatthisratratrat
    //println!("---{}",be[1]);
    for p in 0..bd.len() {
        be.push("-".to_string());
        be.push(bd[p].to_string());
        b = keeptext.split(bd[p].as_str()).map(str::to_string).collect();
        keeptext = String::from("");
        for k in 0..b.len() {
            keeptext.push_str(b[k].as_str());
        }
        //println!("//ss = {}",keeptext);
    }
    b = keeptext.split('-').map(str::to_string).collect();
    for p in 0..b.len() {
        be.push("-".to_string());
        be.push(b[p].to_string());
    }

    let mut keeptext = String::from("");
    for k in 0..be.len() {
        //keeptext.push_str(" ");
        keeptext.push_str(be[k].as_str());
    }
    //println!("be = {}",keeptext);
    be = keeptext.split("-").map(str::to_string).collect();

    let mut keeptext = String::from("");
    for k in 0..be.len() {
        //keeptext.push_str(" ");
        if be[k] != "" {
            keeptext.push_str(be[k].as_str());
            bend.push(be[k].to_string());
            //println!("gg {}",be[k].as_str());
        }
    }
    //println!("be Fainal = {}", keeptext);
    //print!("gg {}",be[0]);
    bend
}
fn count(x: Vec<String>) -> i32 {
    let mut ans: i32 = x.len().try_into().unwrap();
    ans
}

#[cfg(test)]
mod tests {
    use crate::count;
    use crate::unique;
    use crate::words;

    #[test]
    fn count_unique_work() {
        /*let result = 2 + 2;
        assert_eq!(count(), 4);*/
        let v = count(unique(words("this cat this bat this rat")));
        assert_eq!(v, 4);
    }
}

