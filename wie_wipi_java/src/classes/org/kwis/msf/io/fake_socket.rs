use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_runtime::classes::java::{
    io::{InputStream, OutputStream},
    lang::Object,
};
use jvm::{ClassInstanceRef, Jvm, Result as JvmResult};
use wie_jvm_support::{WieJavaClassProto, WieJvmContext};

pub struct FakeSocket;

impl FakeSocket {
    pub fn as_proto() -> WieJavaClassProto {
        WieJavaClassProto {
            name: "net/wie/FakeSocket",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["org/kwis/msf/io/Socket"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "accept",
                    "()Lorg/kwis/msf/io/Socket;",
                    Self::accept,
                    Default::default(),
                ),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new(
                    "getInputStream",
                    "()Ljava/io/InputStream;",
                    Self::get_input_stream,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getMessageCount",
                    "()I",
                    Self::get_message_count,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getMessageMaxLength",
                    "()I",
                    Self::get_message_max_length,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getOutputStream",
                    "()Ljava/io/OutputStream;",
                    Self::get_output_stream,
                    Default::default(),
                ),
                JavaMethodProto::new("isStream", "()Z", Self::is_stream, Default::default()),
                JavaMethodProto::new(
                    "recv",
                    "(Lorg/kwis/msf/io/Message;)V",
                    Self::recv,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "send",
                    "(Lorg/kwis/msf/io/Message;)V",
                    Self::send,
                    Default::default(),
                ),
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
            .invoke_special(&this, "java/lang/Object", "<init>", "()V", ())
            .await?;
        Ok(())
    }

    async fn accept(
        _: &Jvm,
        _: &mut WieJvmContext,
        this: ClassInstanceRef<Self>,
    ) -> JvmResult<ClassInstanceRef<Self>> {
        Ok(this)
    }

    async fn close(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<()> {
        Ok(())
    }

    async fn get_input_stream(
        jvm: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<ClassInstanceRef<InputStream>> {
        let empty = jvm.instantiate_array("B", 0).await?;
        let stream = jvm
            .new_class("java/io/ByteArrayInputStream", "([B)V", (empty,))
            .await?;
        Ok(stream.into())
    }

    async fn get_message_count(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<i32> {
        Ok(0)
    }

    async fn get_message_max_length(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<i32> {
        Ok(0)
    }

    async fn get_output_stream(
        jvm: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<ClassInstanceRef<OutputStream>> {
        let stream = jvm.new_class("net/wie/NullOutputStream", "()V", ()).await?;
        Ok(stream.into())
    }

    async fn is_stream(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<bool> {
        Ok(true)
    }

    async fn recv(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
    ) -> JvmResult<()> {
        Ok(())
    }

    async fn send(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
    ) -> JvmResult<()> {
        Ok(())
    }
}
