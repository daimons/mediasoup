//! Miscellaneous data structures.

use bytes::Bytes;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::any::Any;
use std::borrow::Cow;
use std::fmt;
use std::net::IpAddr;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// Container for arbitrary data attached to Mediasoup entities.
#[derive(Debug, Clone)]
pub struct AppData(Arc<dyn Any + Send + Sync>);

impl Default for AppData {
    fn default() -> Self {
        Self::new(())
    }
}

impl Deref for AppData {
    type Target = Arc<dyn Any + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AppData {
    pub fn new<T: Any + Send + Sync>(app_data: T) -> Self {
        Self(Arc::new(app_data))
    }
}

/// IP to listen on.
///
/// # Notes on usage
/// If you use "0.0.0.0" or "::" as ip value, then you need to also provide `announced_ip`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportListenIp {
    /// Listening IPv4 or IPv6.
    pub ip: IpAddr,
    /// Announced IPv4 or IPv6 (useful when running mediasoup behind NAT with private IP).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announced_ip: Option<IpAddr>,
}

/// ICE role.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IceRole {
    /// The transport is the controlled agent.
    Controlled,
    /// The transport is the controlling agent.
    Controlling,
}

/// ICE parameters.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IceParameters {
    /// ICE username fragment.
    pub username_fragment: String,
    /// ICE password.
    pub password: String,
    /// ICE Lite.
    pub ice_lite: Option<bool>,
}

/// ICE candidate type
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IceCandidateType {
    /// The candidate is a host candidate, whose IP address as specified in the
    /// [`IceCandidate::ip`] property is in fact the true address of the remote peer.
    Host,
    /// The candidate is a server reflexive candidate; the [`IceCandidate::ip`] indicates an
    /// intermediary address assigned by the STUN server to represent the candidate's peer
    /// anonymously.
    Srflx,
    /// The candidate is a peer reflexive candidate; the [`IceCandidate::ip`] is an intermediary
    /// address assigned by the STUN server to represent the candidate's peer anonymously.
    Prflx,
    /// The candidate is a relay candidate, obtained from a TURN server. The relay candidate's IP
    /// address is an address the [TURN](https://developer.mozilla.org/en-US/docs/Glossary/TURN)
    /// server uses to forward the media between the two peers.
    Relay,
}

/// ICE candidate TCP type (always `Passive`).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IceCandidateTcpType {
    Passive,
}

/// Transport protocol.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportProtocol {
    TCP,
    UDP,
}

/// ICE candidate
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IceCandidate {
    /// Unique identifier that allows ICE to correlate candidates that appear on multiple
    /// `transports`.
    pub foundation: String,
    /// The assigned priority of the candidate.
    pub priority: u32,
    /// The IP address of the candidate.
    pub ip: IpAddr,
    /// The protocol of the candidate.
    pub protocol: TransportProtocol,
    /// The port for the candidate.
    pub port: u16,
    /// The type of candidate (always `Host`).
    pub r#type: IceCandidateType,
    /// The type of TCP candidate (always `Passive`).
    pub tcp_type: Option<IceCandidateTcpType>,
}

/// ICE state.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IceState {
    /// No ICE Binding Requests have been received yet.
    New,
    /// Valid ICE Binding Request have been received, but none with USE-CANDIDATE attribute.
    /// Outgoing media is allowed.
    Connected,
    /// ICE Binding Request with USE_CANDIDATE attribute has been received. Media in both directions
    /// is now allowed.
    Completed,
    /// ICE was `Connected` or `Completed` but it has suddenly failed (this can just happen if the
    /// selected tuple has `Tcp` protocol).
    Disconnected,
    /// ICE state when the `transport` has been closed.
    Closed,
}

/// Tuple of local IP/port/protocol + optional remote IP/port.
///
/// # Notes on usage
/// Both `remote_ip` and `remote_port` are unset until the media address of the remote endpoint is
/// known, which happens after calling `transport.connect()` in `PlainTransport` and
/// `PipeTransport`, or via dynamic detection as it happens in `WebRtcTransport` (in which the
/// remote media address is detected by ICE means), or in `PlainTransport` (when using `comedia`
/// mode).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TransportTuple {
    #[serde(rename_all = "camelCase")]
    WithRemote {
        /// Local IP address.
        local_ip: IpAddr,
        /// Local port.
        local_port: u16,
        /// Remote IP address.
        remote_ip: IpAddr,
        /// Remote port.
        remote_port: u16,
        /// Protocol
        protocol: TransportProtocol,
    },
    #[serde(rename_all = "camelCase")]
    LocalOnly {
        /// Local IP address.
        local_ip: IpAddr,
        /// Local port.
        local_port: u16,
        /// Protocol
        protocol: TransportProtocol,
    },
}

/// DTLS state.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DtlsState {
    /// DTLS procedures not yet initiated.
    New,
    /// DTLS connecting.
    Connecting,
    /// DTLS successfully connected (SRTP keys already extracted).
    Connected,
    /// DTLS connection failed.
    Failed,
    /// DTLS state when the `transport` has been closed.
    Closed,
}

/// SCTP state.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SctpState {
    /// SCTP procedures not yet initiated.
    New,
    /// SCTP connecting.
    Connecting,
    /// SCTP successfully connected.
    Connected,
    /// SCTP connection failed.
    Failed,
    /// SCTP state when the transport has been closed.
    Closed,
}

/// DTLS role.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DtlsRole {
    /// The DTLS role is determined based on the resolved ICE role (the `Controlled` role acts as
    /// DTLS client, the `Controlling` role acts as DTLS server).
    /// Since Mediasoup is a ICE Lite implementation it always behaves as ICE `Controlled`.
    Auto,
    /// DTLS client role.
    Client,
    /// DTLS server role.
    Server,
}

impl Default for DtlsRole {
    fn default() -> Self {
        Self::Auto
    }
}

/// The hash function algorithm (as defined in the "Hash function Textual Names" registry initially
/// specified in [RFC 4572](https://tools.ietf.org/html/rfc4572#section-8) Section 8) and its
/// corresponding certificate fingerprint value.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum DtlsFingerprint {
    Sha1 {
        /// Certificate fingerprint value.
        value: [u8; 20],
    },
    Sha224 {
        /// Certificate fingerprint value.
        value: [u8; 28],
    },
    Sha256 {
        /// Certificate fingerprint value.
        value: [u8; 32],
    },
    Sha384 {
        /// Certificate fingerprint value.
        value: [u8; 48],
    },
    Sha512 {
        /// Certificate fingerprint value.
        value: [u8; 64],
    },
}

impl Serialize for DtlsFingerprint {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut rtcp_feedback = serializer.serialize_struct("DtlsFingerprint", 2)?;
        match self {
            DtlsFingerprint::Sha1 { value } => {
                let value = format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    value[0],
                    value[1],
                    value[2],
                    value[3],
                    value[4],
                    value[5],
                    value[6],
                    value[7],
                    value[8],
                    value[9],
                    value[10],
                    value[11],
                    value[12],
                    value[13],
                    value[14],
                    value[15],
                    value[16],
                    value[17],
                    value[18],
                    value[19],
                );
                rtcp_feedback.serialize_field("algorithm", "sha-1")?;
                rtcp_feedback.serialize_field("value", &value)?;
            }
            DtlsFingerprint::Sha224 { value } => {
                let value = format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    value[0],
                    value[1],
                    value[2],
                    value[3],
                    value[4],
                    value[5],
                    value[6],
                    value[7],
                    value[8],
                    value[9],
                    value[10],
                    value[11],
                    value[12],
                    value[13],
                    value[14],
                    value[15],
                    value[16],
                    value[17],
                    value[18],
                    value[19],
                    value[20],
                    value[21],
                    value[22],
                    value[23],
                    value[24],
                    value[25],
                    value[26],
                    value[27],
                );
                rtcp_feedback.serialize_field("algorithm", "sha-224")?;
                rtcp_feedback.serialize_field("value", &value)?;
            }
            DtlsFingerprint::Sha256 { value } => {
                let value = format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}",
                    value[0],
                    value[1],
                    value[2],
                    value[3],
                    value[4],
                    value[5],
                    value[6],
                    value[7],
                    value[8],
                    value[9],
                    value[10],
                    value[11],
                    value[12],
                    value[13],
                    value[14],
                    value[15],
                    value[16],
                    value[17],
                    value[18],
                    value[19],
                    value[20],
                    value[21],
                    value[22],
                    value[23],
                    value[24],
                    value[25],
                    value[26],
                    value[27],
                    value[28],
                    value[29],
                    value[30],
                    value[31],
                );
                rtcp_feedback.serialize_field("algorithm", "sha-256")?;
                rtcp_feedback.serialize_field("value", &value)?;
            }
            DtlsFingerprint::Sha384 { value } => {
                let value = format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    value[0],
                    value[1],
                    value[2],
                    value[3],
                    value[4],
                    value[5],
                    value[6],
                    value[7],
                    value[8],
                    value[9],
                    value[10],
                    value[11],
                    value[12],
                    value[13],
                    value[14],
                    value[15],
                    value[16],
                    value[17],
                    value[18],
                    value[19],
                    value[20],
                    value[21],
                    value[22],
                    value[23],
                    value[24],
                    value[25],
                    value[26],
                    value[27],
                    value[28],
                    value[29],
                    value[30],
                    value[31],
                    value[32],
                    value[33],
                    value[34],
                    value[35],
                    value[36],
                    value[37],
                    value[38],
                    value[39],
                    value[40],
                    value[41],
                    value[42],
                    value[43],
                    value[44],
                    value[45],
                    value[46],
                    value[47],
                );
                rtcp_feedback.serialize_field("algorithm", "sha-384")?;
                rtcp_feedback.serialize_field("value", &value)?;
            }
            DtlsFingerprint::Sha512 { value } => {
                let value = format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:\
                    {:02X}:{:02X}:{:02X}:{:02X}",
                    value[0],
                    value[1],
                    value[2],
                    value[3],
                    value[4],
                    value[5],
                    value[6],
                    value[7],
                    value[8],
                    value[9],
                    value[10],
                    value[11],
                    value[12],
                    value[13],
                    value[14],
                    value[15],
                    value[16],
                    value[17],
                    value[18],
                    value[19],
                    value[20],
                    value[21],
                    value[22],
                    value[23],
                    value[24],
                    value[25],
                    value[26],
                    value[27],
                    value[28],
                    value[29],
                    value[30],
                    value[31],
                    value[32],
                    value[33],
                    value[34],
                    value[35],
                    value[36],
                    value[37],
                    value[38],
                    value[39],
                    value[40],
                    value[41],
                    value[42],
                    value[43],
                    value[44],
                    value[45],
                    value[46],
                    value[47],
                    value[48],
                    value[49],
                    value[50],
                    value[51],
                    value[52],
                    value[53],
                    value[54],
                    value[55],
                    value[56],
                    value[57],
                    value[58],
                    value[59],
                    value[60],
                    value[61],
                    value[62],
                    value[63],
                );
                rtcp_feedback.serialize_field("algorithm", "sha-512")?;
                rtcp_feedback.serialize_field("value", &value)?;
            }
        }
        rtcp_feedback.end()
    }
}

impl<'de> Deserialize<'de> for DtlsFingerprint {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Algorithm,
            Value,
        }

        struct DtlsFingerprintVisitor;

        impl<'de> Visitor<'de> for DtlsFingerprintVisitor {
            type Value = DtlsFingerprint;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    r#"DTLS fingerprint algorithm and value like {"algorithm": "sha-256", "value": "1B:EA:BF:33:B8:11:26:6D:91:AD:1B:A0:16:FD:5D:60:59:33:F7:46:A3:BA:99:2A:1D:04:99:A6:F2:C6:2D:43"}"#,
                )
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut algorithm = None::<Cow<str>>;
                let mut value = None::<Cow<str>>;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Algorithm => {
                            if algorithm.is_some() {
                                return Err(de::Error::duplicate_field("algorithm"));
                            }
                            algorithm = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = map.next_value()?;
                        }
                    }
                }
                let algorithm = algorithm.ok_or_else(|| de::Error::missing_field("algorithm"))?;
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;

                fn parse_as_bytes(input: &str, output: &mut [u8]) -> Result<(), String> {
                    for (i, v) in output.iter_mut().enumerate() {
                        *v = u8::from_str_radix(&input[i * 3..(i * 3) + 2], 16).map_err(
                            |error| {
                                format!(
                                    "Failed to parse value {} as series of hex bytes: {}",
                                    input, error,
                                )
                            },
                        )?;
                    }

                    Ok(())
                }

                match algorithm.as_ref() {
                    "sha-1" => {
                        if value.len() != (20 * 3 - 1) {
                            Err(de::Error::custom(
                                "Value doesn't have correct length for SHA-1",
                            ))
                        } else {
                            let mut value_result = [0u8; 20];
                            parse_as_bytes(value.as_ref(), &mut value_result)
                                .map_err(|error| de::Error::custom(error))?;

                            Ok(DtlsFingerprint::Sha1 {
                                value: value_result,
                            })
                        }
                    }
                    "sha-224" => {
                        if value.len() != (28 * 3 - 1) {
                            Err(de::Error::custom(
                                "Value doesn't have correct length for SHA-224",
                            ))
                        } else {
                            let mut value_result = [0u8; 28];
                            parse_as_bytes(value.as_ref(), &mut value_result)
                                .map_err(|error| de::Error::custom(error))?;

                            Ok(DtlsFingerprint::Sha224 {
                                value: value_result,
                            })
                        }
                    }
                    "sha-256" => {
                        if value.len() != (32 * 3 - 1) {
                            Err(de::Error::custom(
                                "Value doesn't have correct length for SHA-256",
                            ))
                        } else {
                            let mut value_result = [0u8; 32];
                            parse_as_bytes(value.as_ref(), &mut value_result)
                                .map_err(|error| de::Error::custom(error))?;

                            Ok(DtlsFingerprint::Sha256 {
                                value: value_result,
                            })
                        }
                    }
                    "sha-384" => {
                        if value.len() != (48 * 3 - 1) {
                            Err(de::Error::custom(
                                "Value doesn't have correct length for SHA-384",
                            ))
                        } else {
                            let mut value_result = [0u8; 48];
                            parse_as_bytes(value.as_ref(), &mut value_result)
                                .map_err(|error| de::Error::custom(error))?;

                            Ok(DtlsFingerprint::Sha384 {
                                value: value_result,
                            })
                        }
                    }
                    "sha-512" => {
                        if value.len() != (64 * 3 - 1) {
                            Err(de::Error::custom(
                                "Value doesn't have correct length for SHA-512",
                            ))
                        } else {
                            let mut value_result = [0u8; 64];
                            parse_as_bytes(value.as_ref(), &mut value_result)
                                .map_err(|error| de::Error::custom(error))?;

                            Ok(DtlsFingerprint::Sha512 {
                                value: value_result,
                            })
                        }
                    }
                    algorithm => Err(de::Error::unknown_variant(
                        algorithm,
                        &["sha-1", "sha-224", "sha-256", "sha-384", "sha-512"],
                    )),
                }
            }
        }

        const FIELDS: &[&str] = &["algorithm", "value"];
        deserializer.deserialize_struct("DtlsFingerprint", FIELDS, DtlsFingerprintVisitor)
    }
}

/// DTLS parameters.
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub struct DtlsParameters {
    /// DTLS role.
    pub role: DtlsRole,
    /// DTLS fingerprints.
    pub fingerprints: Vec<DtlsFingerprint>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventDirection {
    In,
    Out,
}

/// Container used for sending/receiving messages using `DirectTransport` data producers and data
/// consumers.
#[derive(Clone)]
pub enum WebRtcMessage {
    String(String),
    Binary(Bytes),
    EmptyString,
    EmptyBinary,
}

impl WebRtcMessage {
    // +------------------------------------+-----------+
    // | Value                              | SCTP PPID |
    // +------------------------------------+-----------+
    // | WebRTC String                      | 51        |
    // | WebRTC Binary Partial (Deprecated) | 52        |
    // | WebRTC Binary                      | 53        |
    // | WebRTC String Partial (Deprecated) | 54        |
    // | WebRTC String Empty                | 56        |
    // | WebRTC Binary Empty                | 57        |
    // +------------------------------------+-----------+

    pub(crate) fn new(ppid: u32, payload: Bytes) -> Self {
        // TODO: Make this fallible instead
        match ppid {
            51 => WebRtcMessage::String(String::from_utf8(payload.to_vec()).unwrap()),
            53 => WebRtcMessage::Binary(payload),
            56 => WebRtcMessage::EmptyString,
            57 => WebRtcMessage::EmptyBinary,
            _ => {
                panic!("Bad ppid {}", ppid);
            }
        }
    }

    pub(crate) fn into_ppid_and_payload(self) -> (u32, Bytes) {
        match self {
            WebRtcMessage::String(string) => (51_u32, Bytes::from(string)),
            WebRtcMessage::Binary(binary) => (53_u32, binary),
            WebRtcMessage::EmptyString => (56_u32, Bytes::from_static(b" ")),
            WebRtcMessage::EmptyBinary => (57_u32, Bytes::from(vec![0u8])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dtls_fingerprint() {
        {
            let dtls_fingerprints = &[
                r#"{"algorithm":"sha-1","value":"0D:88:5B:EF:B9:86:F9:66:67:75:7A:C1:7A:78:34:E4:88:DC:44:67"}"#,
                r#"{"algorithm":"sha-224","value":"6E:0C:C7:23:DF:36:E1:C7:46:AB:D7:B1:CE:DD:97:C3:C1:17:25:D6:26:0A:8A:B4:50:F1:3E:BC"}"#,
                r#"{"algorithm":"sha-256","value":"7A:27:46:F0:7B:09:28:F0:10:E2:EC:84:60:B5:87:9A:D9:C8:8B:F3:6C:C5:5D:C3:F3:BA:2C:5B:4F:8A:3A:E3"}"#,
                r#"{"algorithm":"sha-384","value":"D0:B7:F7:3C:71:9F:F4:A1:48:E1:9B:13:25:59:A4:7D:06:BF:E1:1B:DC:0B:8A:8E:45:09:01:22:7E:81:68:EC:DD:B8:DD:CA:1F:F3:F2:E8:15:A5:3C:23:CF:F7:B6:38"}"#,
                r#"{"algorithm":"sha-512","value":"36:8B:9B:CA:2B:01:2B:33:FD:06:95:F2:CC:28:56:69:5B:DD:38:5E:88:32:9A:72:F7:B1:5D:87:9E:64:97:0B:66:A1:C7:6C:BE:4D:CD:83:90:04:AE:20:6C:6D:5F:F0:BD:4C:D9:DD:6E:8A:19:C1:C9:F6:C2:46:C8:08:94:39"}"#,
            ];

            for dtls_fingerprint_str in dtls_fingerprints {
                let dtls_fingerprint =
                    serde_json::from_str::<DtlsFingerprint>(dtls_fingerprint_str).unwrap();
                assert_eq!(
                    dtls_fingerprint_str,
                    &serde_json::to_string(&dtls_fingerprint).unwrap()
                );
            }
        }

        {
            let bad_dtls_fingerprints = &[
                r#"{"algorithm":"sha-1","value":"0D:88:5B:EF:B9:86:F9:66:67::44:67"}"#,
                r#"{"algorithm":"sha-200","value":"6E:0C:C7:23:DF:36:E1:C7:46:AB:D7:B1:CE:DD:97:C3:C1:17:25:D6:26:0A:8A:B4:50:F1:3E:BC"}"#,
            ];

            for dtls_fingerprint_str in bad_dtls_fingerprints {
                assert!(serde_json::from_str::<DtlsFingerprint>(dtls_fingerprint_str).is_err());
            }
        }
    }
}
