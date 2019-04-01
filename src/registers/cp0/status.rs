//! MIPS CP0 Cause register

#[derive(Clone, Copy, Debug)]
pub struct Status {
    pub bits: u32,
}

impl Status {
    register_struct_bit_accessor!(has_ie, set_ie, reset_ie, 0);
    register_struct_bit_accessor!(has_exl, set_exl, reset_exl, 1);
    register_struct_bit_accessor!(has_erl, set_erl, reset_erl, 2);
    register_struct_bit_accessor!(
        is_user_mode, set_user_mode, set_kernel_mode, 4);
    register_struct_bit_accessor!(
        soft_int0_enabled, enable_soft_int0, disable_soft_int0, 8);
    register_struct_bit_accessor!(
        soft_int1_enabled, enable_soft_int1, disable_soft_int1, 9);
    register_struct_bit_accessor!(
        hard_int0_enabled, enable_hard_int0, disable_hard_int0, 10);
    register_struct_bit_accessor!(
        hard_int1_enabled, enable_hard_int1, disable_hard_int1, 11);
    register_struct_bit_accessor!(
        hard_int2_enabled, enable_hard_int2, disable_hard_int2, 12);
    register_struct_bit_accessor!(
        hard_int3_enabled, enable_hard_int3, disable_hard_int3, 13);
    register_struct_bit_accessor!(
        hard_int4_enabled, enable_hard_int4, disable_hard_int4, 14);
    register_struct_bit_accessor!(
        hard_int5_enabled, enable_hard_int5, disable_hard_int5, 15);
    register_struct_bit_accessor!(
        is_bootstrap_mode, enter_bootstrap, leave_bootstrap, 22);
    register_struct_bit_accessor!(has_cu1, set_cu1, reset_cu1, 29);
    register_struct_bit_accessor!(has_cu2, set_cu2, reset_cu2, 30);
    register_struct_bit_accessor!(has_cu3, set_cu3, reset_cu3, 31);
}

register_rw!(12, 0);
register_struct_rw!(Status);

/* Interrupt enable */
register_set_reset_bit!(enable_interrupt, disable_interrupt, 0);

/* Exception level */
register_set_reset_bit!(set_exl, reset_exl, 1);

/* Error level */
register_set_reset_bit!(set_erl, reset_erl, 2);

/* Usermode */
register_set_reset_bit!(set_user_mode, set_kernel_mode, 4);

/* Hard interrupts */
register_set_reset_bit!(enable_hard_int0, disable_hard_int0, 10);
register_set_reset_bit!(enable_hard_int1, disable_hard_int1, 11);
register_set_reset_bit!(enable_hard_int2, disable_hard_int2, 12);
register_set_reset_bit!(enable_hard_int3, disable_hard_int3, 13);
register_set_reset_bit!(enable_hard_int4, disable_hard_int4, 14);
register_set_reset_bit!(enable_hard_int5, disable_hard_int5, 15);

/* Soft interrupts */
register_set_reset_bit!(enable_soft_int0, disable_soft_int0, 8);
register_set_reset_bit!(enable_soft_int1, disable_soft_int1, 9);

/* Controls the location of exception vectors (0 - normal, 1 - bootstrap) */
register_set_reset_bit!(enter_bootstrap, leave_bootstrap, 22);

/* Read/write field (release 6) */
register_set_reset_bit!(set_rw, reset_rw, 28);

/* controls access to coprocessers 1, 2, 3 */
register_set_reset_bit!(set_cu1, reset_cu1, 29);
register_set_reset_bit!(set_cu2, reset_cu2, 30);
register_set_reset_bit!(set_cu3, reset_cu3, 31);
