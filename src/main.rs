fn main() {
    let raw_ptr: *const String;

    // 1. 在一个小作用域内分配内存
    {
        let s = Box::new(String::from("这是原始的机密字符串"));
        raw_ptr = &*s as *const String; // 取得指向堆内存的裸指针
        println!("【1】数据已分配，地址: {:?}", raw_ptr);
        // s 在这里超出作用域，内存被 free
    } 

    println!("【2】s 已经被释放，但 raw_ptr 仍然指向那个地址（悬空指针）");

    // 2. 模拟内存重用：立即分配一个完全不同的对象
    // 操作系统可能会把刚刚释放的那块内存分配给这个新变量
    let _tamper = Box::new(vec![123456789, 987654321]); 
    println!("【3】申请了新内存（一个数组），可能重用了之前的空间");

    // 3. 触发 Use-After-Free
    println!("--- 准备触发非法访问 ---");
    unsafe {
        // 此时 raw_ptr 试图把那一串数字当成 String 来解析
        // 这会导致乱码、内存访问违规或程序崩溃
        println!("【4】读取 UAF 指针内容: {}", *raw_ptr); 
    }
    unsafe {
        // 尝试往一个已经“死亡”的地址写数据
        // 此时 raw_ptr 指向的可能已经是系统的关键数据结构或只读区域
        let mut_ptr = raw_ptr as *mut String;
        *mut_ptr = String::from("丢你楼母"); 
        println!("成功写入？ {}", *mut_ptr);
    }
}