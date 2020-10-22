use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::peek,
    error::ErrorKind,
    Err, IResult,
};

// enum型によるレジスタの表現
// AArch64ではx0からx30までの汎用レジスタがある
#[derive(Debug)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
}

// 算術演算のオペコード
#[derive(Debug)]
pub enum ArithOpcode {
    Add,
    Sub,
    Mul,
    Div,
}

// 分岐のオペコード
#[derive(Debug)]
pub enum BranchOpcode {
    Beq, // ==
    Blt, // <
}

// オペコード
#[derive(Debug)]
pub enum Op {
    Mov(Register, RegOrNum),
    Cmp(Register, Register),
    ArithOp(ArithOpcode, Register, Register, Register),
    BranchOp(BranchOpcode, u64),
}

// レジスタか即値
#[derive(Debug)]
pub enum RegOrNum {
    Reg(Register),
    Num(u64),
}

// これより下はパースしているだけなので読む必要はない
pub fn parse_asm(input: &str) -> IResult<&str, Vec<Op>> {
    let mut v = Vec::new();
    for line in input.lines() {
        let (i, _) = multispace0(line)?;
        if i.is_empty() {
            continue;
        }
        let (i, val) = alt((
            tag("mov"),
            tag("cmp"),
            tag("add"),
            tag("sub"),
            tag("mul"),
            tag("div"),
            tag("b.eq"),
            tag("b.lt"),
        ))(i)?;

        let (i, op) = match val {
            "mov" => parse_mov(i)?,
            "cmp" => parse_cmp(i)?,
            "add" | "sub" | "mul" | "div" => {
                let opcode = get_aop(val).unwrap();
                parse_arith(opcode, i)?
            }
            _ => {
                let opcode = get_brop(val).unwrap();
                parse_branch(opcode, i)?
            }
        };

        let (i, _) = multispace0(i)?;
        if i != "" {
            return Err(Err::Error((i, ErrorKind::Eof)));
        }

        v.push(op);
    }

    Ok(("", v))
}

pub fn get_aop(op: &str) -> Option<ArithOpcode> {
    match op {
        "add" => Some(ArithOpcode::Add),
        "sub" => Some(ArithOpcode::Sub),
        "mul" => Some(ArithOpcode::Mul),
        "div" => Some(ArithOpcode::Div),
        _ => None,
    }
}

pub fn parse_arith(opcode: ArithOpcode, i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    let (i, reg1) = parse_reg(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, reg2) = parse_reg(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, reg3) = parse_reg(i)?;

    Ok((i, Op::ArithOp(opcode, reg1, reg2, reg3)))
}

pub fn get_brop(op: &str) -> Option<BranchOpcode> {
    match op {
        "b.eq" => Some(BranchOpcode::Beq),
        "b.lt" => Some(BranchOpcode::Blt),
        _ => None,
    }
}

pub fn parse_branch(opcode: BranchOpcode, i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    let (i, _) = char('#')(i)?;
    let (i, n) = digit1(i)?;
    Ok((i, Op::BranchOp(opcode, n.parse().unwrap())))
}

pub fn parse_mov(i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    let (i, reg1) = parse_reg(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, c) = peek(alt((char('#'), char('x'))))(i)?;

    if c == '#' {
        let (i, _) = char('#')(i)?;
        let (i, n) = digit1(i)?;
        Ok((i, Op::Mov(reg1, RegOrNum::Num(n.parse().unwrap()))))
    } else {
        let (i, reg2) = parse_reg(i)?;
        Ok((i, Op::Mov(reg1, RegOrNum::Reg(reg2))))
    }
}

pub fn parse_cmp(i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    let (i, reg1) = parse_reg(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, reg2) = parse_reg(i)?;
    Ok((i, Op::Cmp(reg1, reg2)))
}

fn parse_reg(i: &str) -> IResult<&str, Register> {
    let (i, val) = alt((
        alt((
            tag("x0"),
            tag("x1"),
            tag("x2"),
            tag("x3"),
            tag("x4"),
            tag("x5"),
            tag("x6"),
            tag("x7"),
            tag("x8"),
            tag("x9"),
            tag("x10"),
            tag("x11"),
            tag("x12"),
            tag("x13"),
            tag("x14"),
            tag("x15"),
        )),
        alt((
            tag("x16"),
            tag("x17"),
            tag("x18"),
            tag("x19"),
            tag("x20"),
            tag("x21"),
            tag("x22"),
            tag("x23"),
            tag("x24"),
            tag("x25"),
            tag("x26"),
            tag("x27"),
            tag("x28"),
            tag("x29"),
            tag("x30"),
        )),
    ))(i)?;

    match val {
        "x0" => Ok((i, Register::X0)),
        "x1" => Ok((i, Register::X1)),
        "x2" => Ok((i, Register::X2)),
        "x3" => Ok((i, Register::X3)),
        "x4" => Ok((i, Register::X4)),
        "x5" => Ok((i, Register::X5)),
        "x6" => Ok((i, Register::X6)),
        "x7" => Ok((i, Register::X7)),
        "x8" => Ok((i, Register::X8)),
        "x9" => Ok((i, Register::X9)),
        "x10" => Ok((i, Register::X10)),
        "x11" => Ok((i, Register::X11)),
        "x12" => Ok((i, Register::X12)),
        "x13" => Ok((i, Register::X13)),
        "x14" => Ok((i, Register::X14)),
        "x15" => Ok((i, Register::X15)),
        "x16" => Ok((i, Register::X16)),
        "x17" => Ok((i, Register::X17)),
        "x18" => Ok((i, Register::X18)),
        "x19" => Ok((i, Register::X19)),
        "x20" => Ok((i, Register::X20)),
        "x21" => Ok((i, Register::X21)),
        "x22" => Ok((i, Register::X22)),
        "x23" => Ok((i, Register::X23)),
        "x24" => Ok((i, Register::X24)),
        "x25" => Ok((i, Register::X25)),
        "x26" => Ok((i, Register::X26)),
        "x27" => Ok((i, Register::X27)),
        "x28" => Ok((i, Register::X28)),
        "x29" => Ok((i, Register::X29)),
        "x30" => Ok((i, Register::X30)),
        _ => Err(Err::Error(("internal fail", ErrorKind::Tag))),
    }
}
