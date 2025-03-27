use std::arch::asm;

pub fn main() {
    unsafe {
        asm!("nop");
    }

    out();
    in_();
    add();
    direct();
    inlateout();
    // operate();
    mul(10, 20);
    un_damage();
    temp_register();
    call_using_asm();
    hl();
    load_fpu(10);
    control_flow();
    option();
}

fn out() {
    let x: u64;
    unsafe {
        //这里定义寄存器作为输出
        //把寄存器中存储的5写回到x变量的地址中
        //不同架构的处理器的最终汇编指令也会不同
        asm!("mov {},5",out(reg) x);
    }
    assert_eq!(x, 5);
}

fn in_() {
    let i: u64 = 3;
    let o: u64;
    unsafe {
        //这里表示将i变量作为输入值，将输入值输出到变量o
        //mov out(reg) o,in(reg) i
        //这里表示将5加到输出变量o
        //add out(reg) o,5
        asm!(
        "mov {0},{1}",
        "add {0},5",
        out(reg) o,
        in(reg) i
        )
    }
    println!("最终得到值:{}", o);
    assert_eq!(o, 8);
}

fn add() {
    let mut x: u64 = 3;
    //这里的inout，表示变量作为值输入到add指令，并且结果输出到x变量中
    unsafe { asm!("add {0},5",inout(reg) x) }
    assert_eq!(x, 8);
    println!("add函数，最终得到输出值:{x}")
}

fn direct() {
    let x: u64 = 3;
    let y: u64;
    unsafe {
        //这里通过=>，表示x作为输入，y作为输出
        //就实现了让x的值和5相加，再把结果输出给y
        asm!("add {0},5",inout(reg) x=>y)
    }
    println!("x:{x},y:{y}")
}

fn inlateout() {
    let mut a: u64 = 4;
    let b: u64 = 4;
    let c: u64 = 4;
    unsafe {
        //把b的结果加给了a
        //并且有把c的结果加给了a
        asm!("add {0},{1}",
            "add {0},{2}",
        //如果这里用inlateout，并且优化模式，优化器可能会被a和c分配同一个寄存器
        //这样就导致第一条add执行后，a=8
        //第二条指令会变成a+8=16
        //因为优化器觉得a和c在执行add指令之前都能被读取，就觉得把a和c放到一个寄存器里面是安全的(wtf?🐮)
            inout(reg) a,
            in(reg) b,
            in(reg) c);
    }
    println!("a的值:{a}");

    let mut a: u64 = 4;
    let b: u64 = 4;
    unsafe {
        asm!("add {0},{1}",
        //读取完所有输入后，才分配lateout的寄存器
        inlateout(reg) a,
        in(reg) b)
    }

    println!("a:{a}")
}

fn operate() {
    let cmd = 0xd1;
    unsafe {
        //将32位值变量加载到eax寄存器，
        // 并且把eax寄存器内容输出到0x64端口
        //这里的out不同位数需要使用不同的寄存器
        //8位用a1 16位用ax 32位用eax
        asm!("out 0x64,eax",in("eax") cmd);
    }
}

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;
    unsafe {
        //指令内部相乘,十进制转化位二进制，并且内部通过传统类似竖式乘法
        //每一位相乘，再把结果相加，再把不同的位的二进制值存放到不同的寄存器(低64位，高64位)
        //这里担心溢出，指令集内部就把低64位和高64位的数字存放到不同的寄存器中
        asm!("mul {}",
            in(reg) a,
            inlateout("rax") b=>lo,
            lateout("rdx") hi)
    }
    println!("lo:{lo},hi:{hi}");
    ((hi as u128) << 64) + lo as u128
}

/**
Genu
ineI
ntel
*/
fn un_damage() {
    let mut name_buf = [0_u8; 16];

    unsafe {
        //这里ebx是保留寄存器，
        //在64位下，不能直接对ebx进行push pop
        //需要对64位寄存器rbx进行push pop，来保存ebx
        //来达到不破坏ebx保留寄存器的目的
        asm!(
        "push rbx",
        "cpuid",
        "mov [rdi],eax",
        "mov [rdi +4],ebx",
        "mov [rdi +8],edx",
        "mov [rdi +12],ecx",
        "pop rbx",
        in("rdi") name_buf.as_mut_ptr(),
        inout("eax") 0=>_,
        out("ecx") _,
        out("edx") _,
        )
    }
    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("cpu制造商id:{name}");
}

fn temp_register() {
    let mut x: u64 = 4;
    unsafe {
        asm!(
        //x赋值给tmp
        "mov {tmp},{x}",
        //tmp左移1位
        //100
        //1000
        //8 十进制
        "shl {tmp},1",
        //x左移2位
        //100
        //10000
        //16 十进制
        "shl {x},2",
        //tmp加到x
        //8+16 =24
        "add {x},{tmp}",
        x=inout(reg) x,
        tmp=out(reg) _
        )
    }
    //x=24
    println!("x:{x}");
}

extern "C" fn foo(arg: i32) -> i32 {
    println!("foo");
    arg * 2
}

fn call_using_asm() {
    let arg = 6;
    unsafe {
        let result: i32;
        asm!(
        "call {}",
        in(reg) foo,
        in("rdi") arg,
        out("rax") result,
            //这个关键字会告诉rust编译器，根据调用定义，来推断出哪些寄存器会被修改
            //让其调用前保存值，之后恢复值，来达到不影响其他程序操作寄存器的目的
        clobber_abi("C")
        );
        println!("调用asm后:{}", result);
    }
}

fn hl() {
    let mut x: u16 = 0x00ab;
    unsafe {
        //将x的值，低位复制到高位
        //0xabab
        asm!("mov {0:h}, {0:l}",inout(reg_abcd) x);
    }
    println!("0x{:x}", x);
}

fn load_fpu(control: u16) {
    unsafe {
        asm!("fldcw [{}]",in(reg) &control,options(nostack));
    }
}

fn control_flow() {
    let mut a = 0;
    unsafe {
        asm!("mov {0},10",
        "2:",
        "sub {0},1",//向后跳转，继续执行-1操作
        "cmp {0},3",
        "jle 2f",//如果a的值小于等于3,那就向前跳转
        "jmp 2b",//不满足条件，向后跳转
        "2:",//2f向前跳转到这里，循环结束
        "add {0},2",//a+2
        out(reg) a)
    }
    println!("a:{a}");
}

fn option() {
    let mut a: u64 = 3;
    let b: u64 = 3;
    unsafe {
        asm!("add {0}, {1}",
        inlateout(reg) a, in(reg)b,
        //没有可观察的副作用
        //不会读取写入内存
        //不会往栈压入数据
        options(pure,nomem,nostack));
    }
    println!("a:{a}")
}
