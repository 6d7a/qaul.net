//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: node/node.proto

package qaul.rpc.node;

@kotlin.jvm.JvmName("-initializenodeInformation")
public inline fun nodeInformation(block: qaul.rpc.node.NodeInformationKt.Dsl.() -> kotlin.Unit): qaul.rpc.node.NodeOuterClass.NodeInformation =
  qaul.rpc.node.NodeInformationKt.Dsl._create(qaul.rpc.node.NodeOuterClass.NodeInformation.newBuilder()).apply { block() }._build()
public object NodeInformationKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.node.NodeOuterClass.NodeInformation.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.node.NodeOuterClass.NodeInformation.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.node.NodeOuterClass.NodeInformation = _builder.build()

    /**
     * <code>string id_base58 = 1;</code>
     */
    public var idBase58: kotlin.String
      @JvmName("getIdBase58")
      get() = _builder.getIdBase58()
      @JvmName("setIdBase58")
      set(value) {
        _builder.setIdBase58(value)
      }
    /**
     * <code>string id_base58 = 1;</code>
     */
    public fun clearIdBase58() {
      _builder.clearIdBase58()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.node.NodeOuterClass.NodeInformation.copy(block: qaul.rpc.node.NodeInformationKt.Dsl.() -> kotlin.Unit): qaul.rpc.node.NodeOuterClass.NodeInformation =
  qaul.rpc.node.NodeInformationKt.Dsl._create(this.toBuilder()).apply { block() }._build()

