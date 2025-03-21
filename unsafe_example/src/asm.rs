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
        asm!("mul {}",
            in(reg) a,
            inlateout("rax") b=>lo,
            lateout("rdx") hi)
    }
    println!("lo:{lo},hi:{hi}");
    ((hi as u128) << 64) + lo as u128
}
