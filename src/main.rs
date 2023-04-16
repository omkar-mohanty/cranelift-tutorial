use cranelift::{
    codegen::{self, ir::UserFuncName, isa, Context},
    prelude::{settings, FunctionBuilder, FunctionBuilderContext, InstBuilder},
};

use codegen::{
    ir::{types::I64, AbiParam, Function, Signature},
    isa::CallConv,
};
use target_lexicon::Triple;

fn main() {
    let mut sig = Signature::new(CallConv::SystemV);
    sig.params.push(AbiParam::new(I64));
    sig.returns.push(AbiParam::new(I64));

    let mut func = Function::with_name_signature(UserFuncName::default(), sig);

    let mut func_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);

    let block = builder.create_block();

    builder.seal_block(block);
    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);

    let arg = builder.block_params(block)[0];
    let plus_one = builder.ins().iadd_imm(arg, 1);
    builder.ins().return_(&[plus_one]);

    builder.finalize();

    println!("func : {}", func.display());

    let builder = settings::builder();

    let flags = settings::Flags::new(builder);

    let isa = match isa::lookup(Triple::host()) {
        Err(err) => panic!("Error looking up target {}", err),
        Ok(isa_builder) => isa_builder.finish(flags).unwrap(),
    };

    let mut ctx = Context::for_function(func);

    let code = ctx.compile(&*isa).unwrap();

    let len = code.code_buffer().len();

    let mut buffer = memmap2::MmapOptions::new().len(len).map_anon().unwrap();

    buffer.copy_from_slice(code.code_buffer());

    let buffer = buffer.make_exec().unwrap();

    let x = unsafe {
        let code_fn: unsafe extern "sysv64" fn(usize) -> usize =
            std::mem::transmute(buffer.as_ptr());

        code_fn(1)
    };

    println!("out: {}", x);
}
