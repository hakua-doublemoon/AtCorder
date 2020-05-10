
const NUM_OF_AB :usize = 26;
const A_INDEX   :usize = 97;

struct Category {
    //depth    :u32,
    children    :Vec<Category>,
    list        :Vec<String>,
    active_bmp  :u32,
}
impl Category {
    //fn new(depth :u32) -> Category
    fn new() -> Category
    {
        Category {
            //depth:    depth,
            children:   Vec::new(),
            list:       Vec::new(),
            active_bmp: 0,
        }
    }

    fn prepare(&mut self)
    {
        for _ in 0..NUM_OF_AB {
            self.children.push(Category::new());
        }
    }

    fn is_active(&self, key1 :usize, _key2 :usize) -> bool
    {
        let mut ret = false;
        if  (self.active_bmp & (1<<key1)) != 0  {
            //if  (self.children[key1].active_bmp & (1<<key2)) != 0  {
                ret = true;
            //}
        }
        ret
    }

    fn activate(&mut self, key1 :usize, _key2 :usize)
    {
        if  (self.active_bmp & (1<<key1)) == 0  {
            self.children[key1].prepare();
            self.active_bmp |= {1<<key1}; 
        }
        //println!("len = {}", self.children[key1].children[key2].list.len());
    }

    fn search(&self, word :&String) -> bool
    {
        let mut ret = false;
        for i in 0..self.list.len() {
            if  *word == self.list[i]  {
                ret = true;
                break;
            }
        }
        ret
    }

    fn push(&mut self, word :String)
    {
        self.list.push(word);
    }

    fn num_of_nodes(&self) -> u32
    {
        let mut total :u32 = 0;
        for frst in 0..NUM_OF_AB {
            for scnd in 0..NUM_OF_AB {
                if  self.is_active(frst, scnd)  {
                    total += {self.children[frst].children[scnd].list.len() as u32}; 
                }
            }
        }
        total
    }
}

fn index_get(c :char) -> usize
{
    let code = c as usize;
    if  code > A_INDEX  {
        code - A_INDEX
    } else {
        0
    }
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let mut prize :Category = Category::new();
    prize.prepare();

    let num_of_prizes :usize = s.trim().parse::<usize>().ok().unwrap();

    for _ in 0..num_of_prizes {
        let mut s1 = String::new();
        std::io::stdin().read_line(&mut s1).ok();
        //println!("{}", s1);

        let mut si = s1.chars();
        let frst = index_get(si.next().unwrap());
        let scnd = 
            match  si.next()  {
                None    => 0,
                Some(c) => index_get(c),
                //Some(c) => 0,
            };
        if  !prize.is_active(frst, scnd)  {
            prize.activate(frst, scnd);
        }
        if  !prize.children[frst].children[scnd].search(&s1)  {
            prize.children[frst].children[scnd].push(s1);
        }
    }

    println!("{}", prize.num_of_nodes());
}
