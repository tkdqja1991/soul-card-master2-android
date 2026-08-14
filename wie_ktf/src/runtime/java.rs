use alloc::{boxed::Box, collections::BTreeMap, sync::Arc};

use spin::Mutex;

use wie_core_arm::{ArmCore, RegisteredFunction, SvcId};
use wie_util::{Result, WieError};

use crate::runtime::SVC_CATEGORY_JAVA;

pub mod interface;
pub mod jvm_support;

pub type JavaSvcFunctions = Arc<Mutex<BTreeMap<u32, Arc<Box<dyn RegisteredFunction>>>>>;

async fn handle_java_svc(core: &mut ArmCore, svc_functions: &mut JavaSvcFunctions, id: SvcId) -> Result<()> {
    if id.0 == 0x49056b20 {
        let (pc, lr) = core.read_pc_lr()?;
        tracing::warn!(
            "SCM2 JAVA SVC: getInputStream id={:#010x} pc={:#010x} lr={:#010x}",
            id.0,
            pc,
            lr,
        );
    }

    let function = {
        let svc_functions = svc_functions.lock();
        svc_functions
            .get(&id.0)
            .cloned()
            .ok_or_else(|| WieError::FatalError(alloc::format!("Unknown KTF Java SVC id {:#x}", id.0)))?
    };

    function.call(core).await
}

pub fn register_java_svc_handler(core: &mut ArmCore, svc_functions: &JavaSvcFunctions) -> Result<()> {
    core.register_svc_handler(SVC_CATEGORY_JAVA, handle_java_svc, svc_functions)
}
