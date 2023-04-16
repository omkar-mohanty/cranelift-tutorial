use cranelift::{codegen::{self, ir::UserFuncName}, prelude::{FunctionBuilderContext, FunctionBuilder, InstBuilder}};

use codegen::{ir::{types::I64, AbiParam, Function, Signature},
isa::CallConv};

fn main() {
    let mut sig = Signature::new(CallConv::SystemV);
    sig.params.push(AbiParam::new(I64));
    sig.returns.push(AbiParam::new(I64));

    let mut func = Function::with_name_signature(UserFuncName::default(), sig);

    let mut func_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut func,&mut func_ctx);

    let block = builder.create_block();

    builder.seal_block(block);
    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);

    let arg = builder.block_params(block)[0];
    let plus_one = builder.ins().iadd_imm(arg, 1);
    builder.ins().return_(&[plus_one]);

    builder.finalize();

    println!("func : {}", func.display());
}
