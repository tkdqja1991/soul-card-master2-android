use alloc::{vec, vec::Vec};

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, Jvm, Result as JvmResult};
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
                JavaMethodProto::new("write", "([B)V", Self::write_array, Default::default()),
                JavaMethodProto::new("write", "([BII)V", Self::write_array_range, Default::default()),
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

    async fn write_array(
        jvm: &Jvm,
        context: &mut WieJvmContext,
        this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<i8>>,
    ) -> JvmResult<()> {
        let len = jvm.array_length(&buf).await? as i32;
        Self::write_array_range(jvm, context, this, buf, 0, len).await
    }

    async fn write_array_range(
        jvm: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        len: i32,
    ) -> JvmResult<()> {
        if buf.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }

        let array_len = jvm.array_length(&buf).await? as i32;
        if offset < 0 || len < 0 || offset > array_len - len {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }

        let bytes = jvm.load_array::<i8>(&buf, offset as usize, len as usize).await?;
        let unsigned = bytes.iter().map(|b| *b as u8).collect::<Vec<_>>();

        tracing::warn!(
            "SCM2 SOCKET WRITE_ARRAY: offset={} len={} bytes={:?}",
            offset,
            len,
            unsigned,
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
