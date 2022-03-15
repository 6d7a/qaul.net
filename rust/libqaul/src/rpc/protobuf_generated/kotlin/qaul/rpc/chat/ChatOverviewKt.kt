//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/chat/chat.proto

package qaul.rpc.chat;

@kotlin.jvm.JvmSynthetic
public inline fun chatOverview(block: qaul.rpc.chat.ChatOverviewKt.Dsl.() -> kotlin.Unit): qaul.rpc.chat.ChatOuterClass.ChatOverview =
  qaul.rpc.chat.ChatOverviewKt.Dsl._create(qaul.rpc.chat.ChatOuterClass.ChatOverview.newBuilder()).apply { block() }._build()
public object ChatOverviewKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.chat.ChatOuterClass.ChatOverview.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.chat.ChatOuterClass.ChatOverview.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.chat.ChatOuterClass.ChatOverview = _builder.build()

    /**
     * <pre>
     * id of the user
     * </pre>
     *
     * <code>bytes conversation_id = 1;</code>
     */
    public var conversationId: com.google.protobuf.ByteString
      @JvmName("getConversationId")
      get() = _builder.getConversationId()
      @JvmName("setConversationId")
      set(value) {
        _builder.setConversationId(value)
      }
    /**
     * <pre>
     * id of the user
     * </pre>
     *
     * <code>bytes conversation_id = 1;</code>
     */
    public fun clearConversationId() {
      _builder.clearConversationId()
    }

    /**
     * <pre>
     * last message index
     * </pre>
     *
     * <code>uint32 last_message_index = 2;</code>
     */
    public var lastMessageIndex: kotlin.Int
      @JvmName("getLastMessageIndex")
      get() = _builder.getLastMessageIndex()
      @JvmName("setLastMessageIndex")
      set(value) {
        _builder.setLastMessageIndex(value)
      }
    /**
     * <pre>
     * last message index
     * </pre>
     *
     * <code>uint32 last_message_index = 2;</code>
     */
    public fun clearLastMessageIndex() {
      _builder.clearLastMessageIndex()
    }

    /**
     * <pre>
     * name of the conversation
     * </pre>
     *
     * <code>string name = 3;</code>
     */
    public var name: kotlin.String
      @JvmName("getName")
      get() = _builder.getName()
      @JvmName("setName")
      set(value) {
        _builder.setName(value)
      }
    /**
     * <pre>
     * name of the conversation
     * </pre>
     *
     * <code>string name = 3;</code>
     */
    public fun clearName() {
      _builder.clearName()
    }

    /**
     * <pre>
     * time when last message was sent or received
     * </pre>
     *
     * <code>uint64 last_message_at = 4;</code>
     */
    public var lastMessageAt: kotlin.Long
      @JvmName("getLastMessageAt")
      get() = _builder.getLastMessageAt()
      @JvmName("setLastMessageAt")
      set(value) {
        _builder.setLastMessageAt(value)
      }
    /**
     * <pre>
     * time when last message was sent or received
     * </pre>
     *
     * <code>uint64 last_message_at = 4;</code>
     */
    public fun clearLastMessageAt() {
      _builder.clearLastMessageAt()
    }

    /**
     * <pre>
     * unread messages
     * </pre>
     *
     * <code>int32 unread = 5;</code>
     */
    public var unread: kotlin.Int
      @JvmName("getUnread")
      get() = _builder.getUnread()
      @JvmName("setUnread")
      set(value) {
        _builder.setUnread(value)
      }
    /**
     * <pre>
     * unread messages
     * </pre>
     *
     * <code>int32 unread = 5;</code>
     */
    public fun clearUnread() {
      _builder.clearUnread()
    }

    /**
     * <pre>
     * preview text of the last message
     * </pre>
     *
     * <code>string content = 6;</code>
     */
    public var content: kotlin.String
      @JvmName("getContent")
      get() = _builder.getContent()
      @JvmName("setContent")
      set(value) {
        _builder.setContent(value)
      }
    /**
     * <pre>
     * preview text of the last message
     * </pre>
     *
     * <code>string content = 6;</code>
     */
    public fun clearContent() {
      _builder.clearContent()
    }

    /**
     * <pre>
     * sender of the last message
     * </pre>
     *
     * <code>bytes last_message_sender_id = 7;</code>
     */
    public var lastMessageSenderId: com.google.protobuf.ByteString
      @JvmName("getLastMessageSenderId")
      get() = _builder.getLastMessageSenderId()
      @JvmName("setLastMessageSenderId")
      set(value) {
        _builder.setLastMessageSenderId(value)
      }
    /**
     * <pre>
     * sender of the last message
     * </pre>
     *
     * <code>bytes last_message_sender_id = 7;</code>
     */
    public fun clearLastMessageSenderId() {
      _builder.clearLastMessageSenderId()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.chat.ChatOuterClass.ChatOverview.copy(block: qaul.rpc.chat.ChatOverviewKt.Dsl.() -> kotlin.Unit): qaul.rpc.chat.ChatOuterClass.ChatOverview =
  qaul.rpc.chat.ChatOverviewKt.Dsl._create(this.toBuilder()).apply { block() }._build()
