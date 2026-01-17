fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置 proto 文件的根目录
    let proto_root = "Xray-core";

    // 编译所有 proto 文件
    tonic_build::configure()
        .build_server(true) // 启用服务端代码生成（用于实现 gRPC 服务）
        .build_client(true) // 启用客户端代码生成（用于调用 gRPC 服务）
        .compile(
            &[
                // 核心配置
                "Xray-core/core/config.proto",
                // 通用类型
                "Xray-core/common/serial/typed_message.proto",
                "Xray-core/common/log/log.proto",
                "Xray-core/common/net/address.proto",
                "Xray-core/common/net/destination.proto",
                "Xray-core/common/net/network.proto",
                "Xray-core/common/net/port.proto",
                "Xray-core/common/protocol/headers.proto",
                "Xray-core/common/protocol/server_spec.proto",
                "Xray-core/common/protocol/user.proto",
                // App 配置
                "Xray-core/app/commander/config.proto",
                "Xray-core/app/dispatcher/config.proto",
                "Xray-core/app/dns/config.proto",
                "Xray-core/app/dns/fakedns/fakedns.proto",
                "Xray-core/app/log/command/config.proto",
                "Xray-core/app/log/config.proto",
                "Xray-core/app/metrics/config.proto",
                "Xray-core/app/observatory/burst/config.proto",
                "Xray-core/app/observatory/command/command.proto",
                "Xray-core/app/observatory/config.proto",
                "Xray-core/app/policy/config.proto",
                "Xray-core/app/proxyman/command/command.proto",
                "Xray-core/app/proxyman/config.proto",
                "Xray-core/app/reverse/config.proto",
                "Xray-core/app/router/command/command.proto",
                "Xray-core/app/router/config.proto",
                "Xray-core/app/stats/command/command.proto",
                "Xray-core/app/version/config.proto",
                // Proxy 配置
                "Xray-core/proxy/blackhole/config.proto",
                "Xray-core/proxy/dns/config.proto",
                "Xray-core/proxy/dokodemo/config.proto",
                "Xray-core/proxy/freedom/config.proto",
                "Xray-core/proxy/http/config.proto",
                "Xray-core/proxy/hysteria/config.proto",
                "Xray-core/proxy/loopback/config.proto",
                "Xray-core/proxy/shadowsocks/config.proto",
                "Xray-core/proxy/shadowsocks_2022/config.proto",
                "Xray-core/proxy/socks/config.proto",
                "Xray-core/proxy/trojan/config.proto",
                "Xray-core/proxy/tun/config.proto",
                "Xray-core/proxy/vless/account.proto",
                "Xray-core/proxy/vless/encoding/addons.proto",
                "Xray-core/proxy/vless/inbound/config.proto",
                "Xray-core/proxy/vless/outbound/config.proto",
                "Xray-core/proxy/vmess/account.proto",
                "Xray-core/proxy/vmess/inbound/config.proto",
                "Xray-core/proxy/vmess/outbound/config.proto",
                "Xray-core/proxy/wireguard/config.proto",
                // Transport 配置
                "Xray-core/transport/internet/config.proto",
                "Xray-core/transport/internet/finalmask/salamander/config.proto",
                "Xray-core/transport/internet/grpc/config.proto",
                "Xray-core/transport/internet/grpc/encoding/stream.proto",
                "Xray-core/transport/internet/headers/dns/config.proto",
                "Xray-core/transport/internet/headers/http/config.proto",
                "Xray-core/transport/internet/headers/noop/config.proto",
                "Xray-core/transport/internet/headers/srtp/config.proto",
                "Xray-core/transport/internet/headers/tls/config.proto",
                "Xray-core/transport/internet/headers/utp/config.proto",
                "Xray-core/transport/internet/headers/wechat/config.proto",
                "Xray-core/transport/internet/headers/wireguard/config.proto",
                "Xray-core/transport/internet/httpupgrade/config.proto",
                "Xray-core/transport/internet/hysteria/config.proto",
                "Xray-core/transport/internet/kcp/config.proto",
                "Xray-core/transport/internet/reality/config.proto",
                "Xray-core/transport/internet/splithttp/config.proto",
                "Xray-core/transport/internet/tcp/config.proto",
                "Xray-core/transport/internet/tls/config.proto",
                "Xray-core/transport/internet/udp/config.proto",
                "Xray-core/transport/internet/websocket/config.proto",
            ],
            &[proto_root], // include 路径，用于解析相对导入
        )?;

    Ok(())
}
