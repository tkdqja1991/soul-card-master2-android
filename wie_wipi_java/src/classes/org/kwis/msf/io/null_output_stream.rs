use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result as JvmResult};
use wie_jvm_support::{WieJavaClassProto, WieJvmContext};

pub struct NullOutputStream;

impl NullOutputStream {
    pub fn as_proto() -> WieJavaClassProto {
        WieJavaClassProto {
            name: "net/wie/NullOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut WieJvmContext,
        this: ClassInstanceRef<Self>,
    ) -> JvmResult<()> {
        let _: () = jvm
            .invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ())
            .await?;
        Ok(())
    }

    async fn write(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
        byte: i32,
    ) -> JvmResult<()> {
        tracing::warn!(
            "SCM2 SOCKET WRITE: dec={} hex={:02X} char={}",
            byte & 0xff,
            byte & 0xff,
            if (32..=126).contains(&(byte & 0xff)) {
                char::from_u32((byte & 0xff) as u32).unwrap_or('.')
            } else {
                '.'
            }
        );
        Ok(())
    }

    async fn close(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<()> {
        Ok(())
    }
}
