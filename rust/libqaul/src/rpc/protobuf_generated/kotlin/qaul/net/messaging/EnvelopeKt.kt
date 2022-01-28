//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/messaging/messaging.proto

package qaul.net.messaging;

@kotlin.jvm.JvmSynthetic
public inline fun envelope(block: qaul.net.messaging.EnvelopeKt.Dsl.() -> kotlin.Unit): qaul.net.messaging.MessagingOuterClass.Envelope =
  qaul.net.messaging.EnvelopeKt.Dsl._create(qaul.net.messaging.MessagingOuterClass.Envelope.newBuilder()).apply { block() }._build()
public object EnvelopeKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.messaging.MessagingOuterClass.Envelope.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.messaging.MessagingOuterClass.Envelope.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.messaging.MessagingOuterClass.Envelope = _builder.build()

    /**
     * <pre>
     * the qaul ID of the sender
     * </pre>
     *
     * <code>bytes sender_id = 1;</code>
     */
    public var senderId: com.google.protobuf.ByteString
      @JvmName("getSenderId")
      get() = _builder.getSenderId()
      @JvmName("setSenderId")
      set(value) {
        _builder.setSenderId(value)
      }
    /**
     * <pre>
     * the qaul ID of the sender
     * </pre>
     *
     * <code>bytes sender_id = 1;</code>
     */
    public fun clearSenderId() {
      _builder.clearSenderId()
    }

    /**
     * <pre>
     * the qaul ID of the receiver
     * </pre>
     *
     * <code>bytes receiver_id = 2;</code>
     */
    public var receiverId: com.google.protobuf.ByteString
      @JvmName("getReceiverId")
      get() = _builder.getReceiverId()
      @JvmName("setReceiverId")
      set(value) {
        _builder.setReceiverId(value)
      }
    /**
     * <pre>
     * the qaul ID of the receiver
     * </pre>
     *
     * <code>bytes receiver_id = 2;</code>
     */
    public fun clearReceiverId() {
      _builder.clearReceiverId()
    }

    /**
     * <pre>
     * the encrypted message data
     * </pre>
     *
     * <code>bytes data = 3;</code>
     */
    public var data: com.google.protobuf.ByteString
      @JvmName("getData")
      get() = _builder.getData()
      @JvmName("setData")
      set(value) {
        _builder.setData(value)
      }
    /**
     * <pre>
     * the encrypted message data
     * </pre>
     *
     * <code>bytes data = 3;</code>
     */
    public fun clearData() {
      _builder.clearData()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.messaging.MessagingOuterClass.Envelope.copy(block: qaul.net.messaging.EnvelopeKt.Dsl.() -> kotlin.Unit): qaul.net.messaging.MessagingOuterClass.Envelope =
  qaul.net.messaging.EnvelopeKt.Dsl._create(this.toBuilder()).apply { block() }._build()
