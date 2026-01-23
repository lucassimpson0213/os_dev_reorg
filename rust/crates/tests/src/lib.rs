use kernel::idt::makeidt::IdtEntry;

pub fn test_idt_entry() -> u32 {
    let mut idt_entry = IdtEntry::new();
    idt_entry.isr(0);
    return 0;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = test_idt_entry();
        assert_eq!(result, 4);
    }
}
