use crate::fuzz_targets_gen::extract_dep::AllDependencies;
use crate::fuzz_targets_gen::util::Stack;
use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::mir;
use rustc_middle::ty::{self, Ty, TyCtxt};

use super::extract_dep::{extract_arguments, Argument, CalleeDependency};

struct ExtractSequence {
    all_sequence: Vec<Vec<String>>,
}

impl ExtractSequence {
    pub fn new() -> Self {
        ExtractSequence { all_sequence: Vec::new() }
    }

    /// 进行一个深度优先搜索，然后生成遍历序列
    /// 获得函数签名之后，就获得了生成序列的源信息
    pub fn extract_sequence<'tcx>(
        &mut self,
        tcx: TyCtxt<'tcx>,
        all_dependencies: AllDependencies<'tcx>,
        enable: bool,
    ) {
        // 装入所有解析的序列
        let mut all_seq = Vec::new();
        let mut visit_set = FxHashSet::default();

        if !enable {
            return;
        }

        //遍历每一个函数
        for (caller, function) in all_dependencies.functions {
            //避免重复遍历
            if visit_set.contains(&caller) {
                continue;
            }
            visit_set.insert(caller);

            if let Some(_) = caller.as_local() {
                // 测试每一个参数，如果有任何一个不是primitive类型的，都会成功
                let args = function.arguments;
                if args.iter().all(|arg| arg.ty.is_primitive_ty()) {
                    // 能进入这里，说明参数都是基本类型，说明是我们的起始节点
                    // 下面开始dfs

                    let mut func_seq = Vec::new();
                    let mut stack = Stack::new();
                    //dfs的起始节点
                    stack.push(caller);
                    while !stack.is_empty() {
                        println!("123");
                        //下面对于每一个孩子进行遍历
                        let callee_dependency = function.callee_dependencies.clone();
                        for CalleeDependency { callee, callsite, .. } in callee_dependency {
                            use super::extract_dep::Callee;
                            let (crate_name, def_id) = match callee {
                                Callee::DirectCall(def_id) => {
                                    (tcx.crate_name(caller.krate).as_str().to_string(), def_id)
                                }
                                Callee::LocalFunctionPtr(_) => continue, //跳过
                            };

                            if crate_name == "url" {
                                // FIXME: 如果是test crate的api，推入序列
                                func_seq.push(tcx.def_path_str(def_id));

                                if callsite.return_ty.is_primitive_ty() {
                                    all_seq.push(func_seq.clone());
                                    func_seq.clear();
                                }
                            } else {
                                // 否则存入stack供下次遍历
                                stack.push(def_id);
                            }
                        }
                    }

                    // dfs完毕，开始进行结束处理
                    all_seq.push(func_seq);

                    // 结束
                }
            }
        }

        self.all_sequence = all_seq;
    }

    pub fn print_sequence(self) {
        for seq in self.all_sequence.iter().enumerate() {}
    }
}

#[derive(Clone, Debug)]
pub struct CalleeInfo<'tcx> {
    pub callee_path: String,
    pub return_type: Option<Ty<'tcx>>,
    pub argument_type: Vec<Ty<'tcx>>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FunctionInfo<'tcx> {
    id: DefId,
    return_ty: Ty<'tcx>,
    arguments: Vec<Argument<'tcx>>,
}
#[allow(dead_code)]
impl FunctionInfo<'_> {
    pub fn new<'tcx>(tcx: TyCtxt<'tcx>, function: LocalDefId) -> FunctionInfo<'_> {
        let mir = tcx.mir_built(ty::WithOptConstParam {
            did: function,
            const_param_did: tcx.opt_const_param_of(function),
        });
        let mir = mir.borrow();
        let _mir: &mir::Body<'_> = &mir;

        // caller
        let id = function.to_def_id();
        // 返回值
        let return_ty = mir.local_decls[mir::Local::from_usize(0)].ty;
        // 参数
        let arguments = extract_arguments(&mir);

        FunctionInfo { id, return_ty, arguments }
    }
}