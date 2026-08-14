use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
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
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "accept",
                    "()Lorg/kwis/msf/io/Socket;",
                    Self::accept,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getInputStream",
                    "()Ljava/io/InputStream;",
                    Self::get_input_stream,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getMessageCount",
                    "()I",
                    Self::get_message_count,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getMessageMaxLength",
                    "()I",
                    Self::get_message_max_length,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getOutputStream",
                    "()Ljava/io/OutputStream;",
                    Self::get_output_stream,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("isStream", "()Z", Self::is_stream, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "recv",
                    "(Lorg/kwis/msf/io/Message;)V",
                    Self::recv,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "send",
                    "(Lorg/kwis/msf/io/Message;)V",
                    Self::send,
                    MethodAccessFlags::PUBLIC,
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
        tracing::warn!("SCM2 SOCKET: FakeSocket.<init> ENTER");
        tracing::warn!("SCM2 SOCKET: FakeSocket.<init> BEFORE Object.<init>");

        let _: () = jvm
            .invoke_special(&this, "java/lang/Object", "<init>", "()V", ())
            .await?;

        tracing::warn!("SCM2 SOCKET: FakeSocket.<init> AFTER Object.<init>");
        tracing::warn!("SCM2 SOCKET: FakeSocket.<init> EXIT");
        Ok(())
    }

    async fn accept(
        _: &Jvm,
        _: &mut WieJvmContext,
        this: ClassInstanceRef<Self>,
    ) -> JvmResult<ClassInstanceRef<Self>> {
        tracing::warn!("SCM2 SOCKET: accept");
        Ok(this)
    }

    async fn close(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<()> {
        tracing::warn!("SCM2 SOCKET: close");
        Ok(())
    }

    async fn get_input_stream(
        jvm: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<ClassInstanceRef<InputStream>> {
        tracing::warn!("SCM2 SOCKET: getInputStream");
        // SCM2 offline handshake + version-check responses:
        // frame 1: length=1, payload=[0x22]
        // frame 2: length=3, payload=[0x73, 0x00, 0x00]
        let mut response = jvm.instantiate_array("B", 12).await?;
        jvm.store_array(
            &mut response,
            0,
            [
                0_i8, 0_i8, 0_i8, 1_i8, 0x22_i8,
                0_i8, 0_i8, 0_i8, 3_i8, 0x73_i8, 0_i8, 0_i8,
            ],
        )
        .await?;

        tracing::warn!(
            "SCM2 SOCKET: injected responses [00 00 00 01 22] [00 00 00 03 73 00 00]"
        );

        let stream = jvm
            .new_class("java/io/ByteArrayInputStream", "([B)V", (response,))
            .await?;

        tracing::warn!("SCM2 SOCKET: FakeSocket input stream object={stream:?}");

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
        tracing::warn!("SCM2 SOCKET: getOutputStream");
        let stream = jvm.new_class("net/wie/NullOutputStream", "()V", ()).await?;
        Ok(stream.into())
    }

    async fn is_stream(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
    ) -> JvmResult<bool> {
        tracing::warn!("SCM2 SOCKET: isStream");
        Ok(true)
    }

    async fn recv(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
    ) -> JvmResult<()> {
        tracing::warn!("SCM2 SOCKET: recv");
        Ok(())
    }

    async fn send(
        _: &Jvm,
        _: &mut WieJvmContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
    ) -> JvmResult<()> {
        tracing::warn!("SCM2 SOCKET: send");
        Ok(())
    }
}
