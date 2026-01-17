//! Xray gRPC 客户端和服务端代码
//!
//! 此模块包含从 Xray-core protobuf 定义生成的 Rust 代码。

// 包含由 build.rs 生成的 protobuf 代码
// tonic-build 会将生成的代码放在 OUT_DIR 中
// 每个 proto 文件会生成一个对应的 Rust 模块

// 核心配置
pub mod core {
    include!(concat!(env!("OUT_DIR"), "/xray.core.rs"));
}

// 通用类型
pub mod common {
    pub mod serial {
        include!(concat!(env!("OUT_DIR"), "/xray.common.serial.rs"));
    }
    pub mod log {
        include!(concat!(env!("OUT_DIR"), "/xray.common.log.rs"));
    }
    pub mod net {
        include!(concat!(env!("OUT_DIR"), "/xray.common.net.rs"));
    }
    pub mod protocol {
        include!(concat!(env!("OUT_DIR"), "/xray.common.protocol.rs"));
    }
}

// App 配置
pub mod app {
    pub mod commander {
        include!(concat!(env!("OUT_DIR"), "/xray.app.commander.rs"));
    }
    pub mod dispatcher {
        include!(concat!(env!("OUT_DIR"), "/xray.app.dispatcher.rs"));
    }
    pub mod dns {
        include!(concat!(env!("OUT_DIR"), "/xray.app.dns.rs"));
        pub mod fakedns {
            include!(concat!(env!("OUT_DIR"), "/xray.app.dns.fakedns.rs"));
        }
    }
    pub mod log {
        include!(concat!(env!("OUT_DIR"), "/xray.app.log.rs"));
        pub mod command {
            include!(concat!(env!("OUT_DIR"), "/xray.app.log.command.rs"));
        }
    }
    pub mod metrics {
        include!(concat!(env!("OUT_DIR"), "/xray.app.metrics.rs"));
    }
    pub mod observatory {
        include!(concat!(env!("OUT_DIR"), "/xray.core.app.observatory.rs"));
        pub mod burst {
            include!(concat!(env!("OUT_DIR"), "/xray.core.app.observatory.burst.rs"));
        }
        pub mod command {
            include!(concat!(env!("OUT_DIR"), "/xray.core.app.observatory.command.rs"));
        }
    }
    pub mod policy {
        include!(concat!(env!("OUT_DIR"), "/xray.app.policy.rs"));
    }
    pub mod proxyman {
        include!(concat!(env!("OUT_DIR"), "/xray.app.proxyman.rs"));
        pub mod command {
            include!(concat!(env!("OUT_DIR"), "/xray.app.proxyman.command.rs"));
        }
    }
    pub mod reverse {
        include!(concat!(env!("OUT_DIR"), "/xray.app.reverse.rs"));
    }
    pub mod router {
        include!(concat!(env!("OUT_DIR"), "/xray.app.router.rs"));
        pub mod command {
            include!(concat!(env!("OUT_DIR"), "/xray.app.router.command.rs"));
        }
    }
    pub mod stats {
        pub mod command {
            include!(concat!(env!("OUT_DIR"), "/xray.app.stats.command.rs"));
        }
    }
    pub mod version {
        include!(concat!(env!("OUT_DIR"), "/xray.app.version.rs"));
    }
}

// Proxy 配置
pub mod proxy {
    pub mod blackhole {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.blackhole.rs"));
    }
    pub mod dns {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.dns.rs"));
    }
    pub mod dokodemo {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.dokodemo.rs"));
    }
    pub mod freedom {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.freedom.rs"));
    }
    pub mod http {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.http.rs"));
    }
    pub mod hysteria {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.hysteria.rs"));
    }
    pub mod loopback {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.loopback.rs"));
    }
    pub mod shadowsocks {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.shadowsocks.rs"));
    }
    pub mod shadowsocks_2022 {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.shadowsocks_2022.rs"));
    }
    pub mod socks {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.socks.rs"));
    }
    pub mod trojan {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.trojan.rs"));
    }
    pub mod tun {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.tun.rs"));
    }
    pub mod vless {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.vless.rs"));
        pub mod encoding {
            include!(concat!(env!("OUT_DIR"), "/xray.proxy.vless.encoding.rs"));
        }
        pub mod inbound {
            include!(concat!(env!("OUT_DIR"), "/xray.proxy.vless.inbound.rs"));
        }
        pub mod outbound {
            include!(concat!(env!("OUT_DIR"), "/xray.proxy.vless.outbound.rs"));
        }
    }
    pub mod vmess {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.vmess.rs"));
        pub mod inbound {
            include!(concat!(env!("OUT_DIR"), "/xray.proxy.vmess.inbound.rs"));
        }
        pub mod outbound {
            include!(concat!(env!("OUT_DIR"), "/xray.proxy.vmess.outbound.rs"));
        }
    }
    pub mod wireguard {
        include!(concat!(env!("OUT_DIR"), "/xray.proxy.wireguard.rs"));
    }
}

// Transport 配置
pub mod transport {
    pub mod internet {
        include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.rs"));
        pub mod udpmask {
            pub mod salamander {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/xray.transport.internet.udpmask.salamander.rs"
                ));
            }
        }
        pub mod grpc {
            pub mod encoding {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.grpc.encoding.rs"));
            }
        }
        pub mod headers {
            pub mod dns {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.headers.dns.rs"));
            }
            pub mod http {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.headers.http.rs"));
            }
            pub mod noop {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.headers.noop.rs"));
            }
            pub mod srtp {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.headers.srtp.rs"));
            }
            pub mod tls {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.headers.tls.rs"));
            }
            pub mod utp {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.headers.utp.rs"));
            }
            pub mod wechat {
                include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.headers.wechat.rs"));
            }
            pub mod wireguard {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/xray.transport.internet.headers.wireguard.rs"
                ));
            }
        }
        pub mod httpupgrade {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.httpupgrade.rs"));
        }
        pub mod hysteria {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.hysteria.rs"));
        }
        pub mod kcp {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.kcp.rs"));
        }
        pub mod reality {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.reality.rs"));
        }
        pub mod splithttp {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.splithttp.rs"));
        }
        pub mod tcp {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.tcp.rs"));
        }
        pub mod tls {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.tls.rs"));
        }
        pub mod udp {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.udp.rs"));
        }
        pub mod websocket {
            include!(concat!(env!("OUT_DIR"), "/xray.transport.internet.websocket.rs"));
        }
    }
}

// 重新导出常用的类型
pub use common::serial::TypedMessage;
pub use core::*;

// 重新导出 gRPC 服务客户端
pub use app::log::command::logger_service_client::LoggerServiceClient;
pub use app::observatory::command::observatory_service_client::ObservatoryServiceClient;
pub use app::proxyman::command::handler_service_client::HandlerServiceClient;
pub use app::router::command::routing_service_client::RoutingServiceClient;
pub use app::stats::command::stats_service_client::StatsServiceClient;
pub use transport::internet::grpc::encoding::grpc_service_client::GrpcServiceClient;

// 重新导出 gRPC 服务服务端 trait
pub use app::log::command::logger_service_server::LoggerService;
pub use app::observatory::command::observatory_service_server::ObservatoryService;
pub use app::proxyman::command::handler_service_server::HandlerService;
pub use app::router::command::routing_service_server::RoutingService;
pub use app::stats::command::stats_service_server::StatsService;
pub use transport::internet::grpc::encoding::grpc_service_server::GrpcService;
