use std::mem::size_of;

use inkwell::builder::Builder;
use inkwell::types::ArrayType;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::IntPredicate;
use mir_llvm::{CodegenCx, MemLoc};
type Word = u32;
const WORD_BYTES: u32 = size_of::<Word>() as u32;
const WORD_BITS: u32 = WORD_BYTES * 8;

fn word_index_and_mask(pos: u32) -> (u32, u32) {
    let word_index = pos / WORD_BITS;
    let mask = 1 << (pos % WORD_BITS);
    (word_index, mask)
}

fn word_cnt(len: u32) -> u32 {
    (len + WORD_BITS - 1) / WORD_BITS
}

pub fn arr_ty<'ll>(len: u32, cx: &CodegenCx<'_, 'll>) -> ArrayType<'ll> {
    cx.ty_array(cx.ty_int().into(), word_cnt(len))
}
pub unsafe fn word_ptr_and_mask<'ll>(
    cx: &CodegenCx<'_, 'll>,
    pos: u32,
    arr_ptr: PointerValue<'ll>,
    arr_ty: ArrayType<'ll>,
    builder: &Builder<'ll>,
) -> (PointerValue<'ll>, IntValue<'ll>) {
    let (idx, mask) = word_index_and_mask(pos);
    let zero = cx.const_int(0);
    let pos = cx.const_unsigned_int(idx);

    // Build GEP to get pointer to word in array
    let indices = [zero.into(), pos.into()];
    let word_ptr = builder.build_gep(arr_ty, arr_ptr, &indices, "word_ptr").unwrap();

    let mask = cx.const_unsigned_int(mask);
    (word_ptr, mask)
}

pub unsafe fn is_set<'ll>(
    cx: &CodegenCx<'_, 'll>,
    pos: u32,
    arr_ptr: PointerValue<'ll>,
    arr_ty: ArrayType<'ll>,
    builder: &Builder<'ll>,
) -> IntValue<'ll> {
    let (ptr, mask) = word_ptr_and_mask(cx, pos, arr_ptr, arr_ty, builder);
    let word = builder.build_load(cx.ty_int(), ptr, "word").unwrap().into_int_value();
    let is_set = builder.build_and(word, mask, "is_set").unwrap();
    let zero = cx.const_int(0);
    builder.build_int_compare(IntPredicate::NE, is_set, zero, "cmp").unwrap()
}

pub unsafe fn set_bit<'ll>(
    cx: &CodegenCx<'_, 'll>,
    pos: u32,
    arr_ptr: PointerValue<'ll>,
    arr_ty: ArrayType<'ll>,
    builder: &Builder<'ll>,
) {
    let (ptr, mask) = word_ptr_and_mask(cx, pos, arr_ptr, arr_ty, builder);
    let word = builder.build_load(cx.ty_int(), ptr, "word").unwrap().into_int_value();
    let new_word = builder.build_or(word, mask, "new_word").unwrap();
    builder.build_store(ptr, new_word).unwrap();
}

pub unsafe fn is_flag_set_mem<'ll>(
    cx: &CodegenCx<'_, 'll>,
    flag: u32,
    val: &MemLoc<'ll>,
    builder: &Builder<'ll>,
) -> IntValue<'ll> {
    let loaded = val.read(builder).into_int_value();
    is_flag_set(cx, flag, loaded, builder)
}

// pub unsafe fn is_flag_unset_mem<'ll>(
//     cx: &CodegenCx<'_, 'll>,
//     flag: u32,
//     val: MemLoc<'ll>,
//     llbuilder: &llvm_sys::LLVMBuilder,
// ) -> &'ll llvm_sys::LLVMValue {
//     is_flag_unset(cx, flag, val.read(llbuilder), llbuilder)
// }

pub unsafe fn is_flag_set<'ll>(
    cx: &CodegenCx<'_, 'll>,
    flag: u32,
    val: IntValue<'ll>,
    builder: &Builder<'ll>,
) -> IntValue<'ll> {
    let mask = cx.const_unsigned_int(flag);
    let bits = builder.build_and(mask, val, "bits").unwrap();
    builder.build_int_compare(IntPredicate::NE, bits, cx.const_int(0), "flag_set").unwrap()
}

pub unsafe fn is_flag_unset<'ll>(
    cx: &CodegenCx<'_, 'll>,
    flag: u32,
    val: IntValue<'ll>,
    builder: &Builder<'ll>,
) -> IntValue<'ll> {
    let mask = cx.const_unsigned_int(flag);
    let bits = builder.build_and(mask, val, "bits").unwrap();
    builder.build_int_compare(IntPredicate::EQ, bits, cx.const_int(0), "flag_unset").unwrap()
}
