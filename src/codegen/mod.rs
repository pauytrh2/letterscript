use crate::parse::Program;

pub fn to_asm(parse_tree: Program) -> String {
    let mut output = String::new();

    let function_name = parse_tree.function_name;

    output.push_str("global _start\n");
    output.push_str(format!("global _{function_name}\n\n").as_str());
    output.push_str("_start:\n");
    output.push_str(format!("    call _{function_name}\n\n").as_str());

    output.push_str(format!("_{function_name}:\n").as_str());
    output.push_str("    mov eax, 60\n");
    output.push_str(format!("    mov edi, {}\n", parse_tree.return_value).as_str());
    output.push_str("    syscall\n\n");

    output
}
